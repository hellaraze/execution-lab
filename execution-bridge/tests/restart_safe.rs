use execution_bridge::*;
use el_core::event::{Event, EventId, EventPayload, EventType, Exchange};
use el_core::instrument::InstrumentKey;
use el_core::time::{Timestamp, TimeSource};
use eventlog::{EventLogReader, EventLogWriter};
use uuid::Uuid;
use std::collections::HashMap;

fn mk_event(id: EventId) -> Event {
    Event {
        id,
        event_type: EventType::Connectivity,
        exchange: Exchange::Binance,
        symbol: "BTCUSDT".to_string(),
        instrument: InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
        ts_exchange: None,
        ts_recv: Timestamp::new(1, TimeSource::Receive),
        ts_proc: Timestamp::new(2, TimeSource::Process),
        seq: None,
        schema_version: 1,
        integrity_flags: vec![],
        payload: EventPayload::Connectivity { status: "ok".to_string() },
        meta: HashMap::new(),
    }
}

fn count_id(path: &std::path::Path, id: EventId) -> usize {
    let mut r = EventLogReader::open(path).unwrap();
    let mut n = 0usize;
    loop {
        let (env, payload) = match r.next().unwrap() {
            Some(v) => v,
            None => break,
        };
        if env.kind != "event" { continue; }
        let e: Event = serde_json::from_slice(&payload).unwrap();
        if e.id == id { n += 1; }
    }
    n
}

#[test]
fn restart_safe_exactly_once() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("outbox.log");

    let id = Uuid::new_v4();
    let ev = mk_event(id);

    {
        let w = EventLogWriter::open_append(&path, "exec", eventlog::writer::Durability::Buffered).unwrap();
        let mut b1 = Bridge::new(w);
        b1.publish_once(ev.clone()).unwrap();
    }

    // reopen and try to publish again
    {
        let mut b2 = Bridge::open_dedup(&path, "exec", eventlog::writer::Durability::Buffered).unwrap();
        b2.publish_once(ev.clone()).unwrap();
    }

    assert_eq!(count_id(&path, ev.id), 1);
}
