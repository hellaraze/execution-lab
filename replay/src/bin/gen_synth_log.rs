use anyhow::Result;
use el_core::event::{Event, EventPayload, EventType, Exchange};
use el_core::instrument::InstrumentKey;
use el_core::time::{TimeSource, Timestamp};
use eventlog::writer::Durability;
use eventlog::EventLogWriter;
use std::collections::HashMap;
use uuid::Uuid;

fn ts(nanos: i64) -> Timestamp {
    Timestamp::new(nanos, TimeSource::Process)
}

fn main() -> Result<()> {
    let out_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "replay/tests/data/golden_events_book.log".to_string());

    let mut w = EventLogWriter::open_append(&out_path, "golden", Durability::Buffered)?;

    // 1) Snapshot
    let snap = Event {
        id: Uuid::new_v4(),
        event_type: EventType::BookSnapshot,
        exchange: Exchange::Binance,
        symbol: "BTCUSDT".to_string(),
        instrument: InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
        ts_exchange: None,
        ts_recv: ts(1),
        ts_proc: ts(1),
        seq: Some(1),
        schema_version: 1,
        integrity_flags: vec![],
        payload: EventPayload::BookSnapshot {
            bids: vec![(100.0, 1.0), (99.5, 2.0), (99.0, 3.0)],
            asks: vec![(100.5, 1.5), (101.0, 2.5), (101.5, 3.5)],
        },
        meta: HashMap::new(),
    };
    w.write(&snap)?;

    // 2) Deltas: deterministic pattern
    for i in 0..250u64 {
        let k = i + 2;
        let bid_px = 100.0 - (i % 20) as f64 * 0.5;
        let ask_px = 100.5 + (i % 20) as f64 * 0.5;

        let bid_qty = if i % 17 == 0 {
            0.0
        } else {
            1.0 + (i % 5) as f64 * 0.1
        };
        let ask_qty = if i % 19 == 0 {
            0.0
        } else {
            1.5 + (i % 7) as f64 * 0.1
        };

        let ev = Event {
            id: Uuid::new_v4(),
            event_type: EventType::BookDelta,
            exchange: Exchange::Binance,
            symbol: "BTCUSDT".to_string(),
            instrument: InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
            ts_exchange: None,
            ts_recv: ts(1 + k as i64),
            ts_proc: ts(1 + k as i64),
            seq: Some(k),
            schema_version: 1,
            integrity_flags: vec![],
            payload: EventPayload::BookDelta {
                bids: vec![(bid_px, bid_qty)],
                asks: vec![(ask_px, ask_qty)],
            },
            meta: HashMap::new(),
        };
        w.write(&ev)?;
    }

    w.flush()?;
    println!("wrote synth log: {}", out_path);
    Ok(())
}
