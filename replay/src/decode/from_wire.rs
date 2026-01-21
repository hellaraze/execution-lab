use crate::decode::DecodeError;
use crate::wire::{BookLevels, TickerBbo, WireEvent, WireTs};
use el_core::event::{Event, EventPayload, EventType, Exchange};
use el_core::instrument::InstrumentKey;
use el_core::time::{TimeSource, Timestamp};
use uuid::Uuid;

const EVENT_ID_NAMESPACE: Uuid = Uuid::from_bytes([0x45,0x4c,0x2d,0x45,0x56,0x54,0x2d,0x49,0x44,0x2d,0x4e,0x53,0x50,0x41,0x43,0x45]);

fn parse_time_source(s: &str) -> Option<TimeSource> {
    match s {
        "Exchange" => Some(TimeSource::Exchange),
        "Receive" => Some(TimeSource::Receive),
        "Process" => Some(TimeSource::Process),
        _ => None,
    }
}

fn ts_from_wire(ts: &WireTs) -> Result<Timestamp, DecodeError> {
    let src = parse_time_source(&ts.source).ok_or(DecodeError::Invalid("ts.source"))?;
    // wire nanos is u64, core expects i64
    let nanos_i64: i64 = ts.nanos.try_into().map_err(|_| DecodeError::Invalid("ts.nanos"))?;
    Ok(Timestamp::new(nanos_i64, src))
}

fn ts_opt_from_wire(ts: &Option<WireTs>) -> Result<Option<Timestamp>, DecodeError> {
    match ts {
        None => Ok(None),
        Some(x) => Ok(Some(ts_from_wire(x)?)),
    }
}


fn event_id_from_wire(w: &WireEvent, event_type: &EventType) -> Uuid {
    // Deterministic ID for replay/audit: same wire -> same id
    // NOTE: payload is intentionally excluded (can be large); we key by stream+type+seq+timestamps.
    let mut key = String::new();
    key.push_str(&w.exchange);
    key.push('|');
    key.push_str(&w.symbol);
    key.push('|');
    key.push_str(match event_type {
        EventType::BookSnapshot => "BookSnapshot",
        EventType::BookDelta => "BookDelta",
        EventType::Trade => "Trade",
        EventType::TickerBbo => "TickerBbo",
        _ => "Other",
    });
    key.push('|');
    if let Some(seq) = w.seq {
        key.push_str(&seq.to_string());
    }
    key.push('|');
    key.push_str(&w.schema_version.to_string());
    key.push('|');
    key.push_str(&w.ts_recv.nanos.to_string());
    key.push('|');
    key.push_str(&w.ts_proc.nanos.to_string());
    if let Some(tsx) = &w.ts_exchange {
        key.push('|');
        key.push_str(&tsx.nanos.to_string());
    }

    Uuid::new_v5(&EVENT_ID_NAMESPACE, key.as_bytes())
}

fn exchange_from_str(s: &str) -> Exchange {
    match s {
        "Binance" => Exchange::Binance,
        "Okx" => Exchange::Okx,
        "Bybit" => Exchange::Bybit,
        other => Exchange::Other(other.to_string()),
    }
}

pub fn decode_event(w: WireEvent) -> Result<Event, DecodeError> {
    let event_type = match w.event_type.as_str() {
        "BookSnapshot" => EventType::BookSnapshot,
        "BookDelta" => EventType::BookDelta,
        "Trade" => EventType::Trade,
        "TickerBbo" => EventType::TickerBbo,
        other => return Err(DecodeError::Unsupported(other.to_string())),
    };

    let payload: EventPayload = match event_type {
        EventType::BookSnapshot => {
            let v = w
                .payload
                .get("BookSnapshot")
                .ok_or(DecodeError::PayloadKeyMismatch {
                    event_type: w.event_type.clone(),
                    expected: "BookSnapshot",
                })?
                .clone();
            let lv: BookLevels = serde_json::from_value(v).map_err(|_| DecodeError::Invalid("payload.BookSnapshot"))?;
            EventPayload::BookSnapshot { bids: lv.bids, asks: lv.asks }
        }
        EventType::BookDelta => {
            let v = w
                .payload
                .get("BookDelta")
                .ok_or(DecodeError::PayloadKeyMismatch {
                    event_type: w.event_type.clone(),
                    expected: "BookDelta",
                })?
                .clone();
            let lv: BookLevels = serde_json::from_value(v).map_err(|_| DecodeError::Invalid("payload.BookDelta"))?;
            EventPayload::BookDelta { bids: lv.bids, asks: lv.asks }
        }
        EventType::TickerBbo => {
            let v = w
                .payload
                .get("TickerBbo")
                .ok_or(DecodeError::PayloadKeyMismatch {
                    event_type: w.event_type.clone(),
                    expected: "TickerBbo",
                })?
                .clone();
            let b: TickerBbo = serde_json::from_value(v).map_err(|_| DecodeError::Invalid("payload.TickerBbo"))?;
            EventPayload::TickerBbo { bid: b.bid, ask: b.ask }
        }
        // not in your sample logs yet; keep strict
        _ => return Err(DecodeError::Unsupported(w.event_type)),
    };

    let symbol = w.symbol.clone();

    Ok(Event {
        id: event_id_from_wire(&w, &event_type),
        event_type,
        exchange: exchange_from_str(&w.exchange),
        symbol: w.symbol,

        instrument: InstrumentKey::new(exchange_from_str(&w.exchange), symbol),

        ts_exchange: ts_opt_from_wire(&w.ts_exchange)?,
        ts_recv: ts_from_wire(&w.ts_recv)?,
        ts_proc: ts_from_wire(&w.ts_proc)?,

        seq: w.seq,
        schema_version: w.schema_version,
        integrity_flags: w.integrity_flags,

        payload,
        meta: w.meta,
    })
}
