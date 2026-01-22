use anyhow::Result;
use el_core::event::{Event, EventPayload, EventType, Exchange};
use el_core::time::{TimeSource, Timestamp};
use el_core::instrument::InstrumentKey;
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
        .unwrap_or_else(|| "replay/tests/data/exec_events.log".to_string());

    let _ = std::fs::remove_file(&out_path);

    let mut w = EventLogWriter::open_append(&out_path, "exec", Durability::Buffered)?;

    // OrderSubmit (id=42)
    let submit = Event {
        id: Uuid::new_v4(),
        event_type: EventType::OrderSubmit,
        exchange: Exchange::Binance,
        symbol: "BTCUSDT".to_string(),
        instrument: InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
        ts_exchange: None,
        ts_recv: ts(5),
        ts_proc: ts(5),
        seq: Some(5),
        schema_version: 1,
        integrity_flags: vec![],
        payload: EventPayload::OrderSubmit {
            order_id: "42".to_string(),
            side: "BUY".to_string(),
            price: 123.0,
            qty: 0.5,
        },
        meta: HashMap::new(),
    };
    w.write(&submit)?;

    // OrderAck (id=42)
    let ack = Event {
        id: Uuid::new_v4(),
        event_type: EventType::OrderAck,
        exchange: Exchange::Binance,
        symbol: "BTCUSDT".to_string(),
        instrument: InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
        ts_exchange: None,
        ts_recv: ts(1),
        ts_proc: ts(1),
        seq: Some(1),
        schema_version: 1,
        integrity_flags: vec![],
        payload: EventPayload::OrderAck { order_id: "42".to_string() },
        meta: HashMap::new(),
    };
    w.write(&ack)?;

    // Fill (id=42)
    let fill = Event {
        id: Uuid::new_v4(),
        event_type: EventType::Fill,
        exchange: Exchange::Binance,
        symbol: "BTCUSDT".to_string(),
        instrument: InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
        ts_exchange: None,
        ts_recv: ts(2),
        ts_proc: ts(2),
        seq: Some(2),
        schema_version: 1,
        integrity_flags: vec![],
        payload: EventPayload::Fill {
            order_id: "42".to_string(),
            fill_id: "f1".to_string(),
            price: 123.0,
            qty: 0.5,
        },
        meta: HashMap::new(),
    };
    w.write(&fill)?;

    // CancelRequest (id=42)
    let cancel_req = Event {
        id: Uuid::new_v4(),
        event_type: EventType::CancelRequest,
        exchange: Exchange::Binance,
        symbol: "BTCUSDT".to_string(),
        instrument: InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
        ts_exchange: None,
        ts_recv: ts(4),
        ts_proc: ts(4),
        seq: Some(4),
        schema_version: 1,
        integrity_flags: vec![],
        payload: EventPayload::CancelRequest { order_id: "42".to_string() },
        meta: HashMap::new(),
    };
    w.write(&cancel_req)?;

    // CancelAck (id=42)
    let cancel = Event {
        id: Uuid::new_v4(),
        event_type: EventType::CancelAck,
        exchange: Exchange::Binance,
        symbol: "BTCUSDT".to_string(),
        instrument: InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
        ts_exchange: None,
        ts_recv: ts(3),
        ts_proc: ts(3),
        seq: Some(3),
        schema_version: 1,
        integrity_flags: vec![],
        payload: EventPayload::CancelAck { order_id: "42".to_string() },
        meta: HashMap::new(),
    };
    w.write(&cancel)?;

    w.flush()?;
    println!("wrote exec log: {}", out_path);
    Ok(())
}
