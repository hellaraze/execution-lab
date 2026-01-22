use execution_bridge::*;
use el_core::event::{Event, EventId, EventPayload, EventType, Exchange};
use el_core::instrument::InstrumentKey;
use el_core::time::{Timestamp, TimeSource};
use eventlog::{EventLogReader, EventLogWriter};
use base64::Engine;
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

#[test]
fn exactly_once_append_is_idempotent_by_event_id() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("exec_outbox.log");

    let w = EventLogWriter::open_append(&path, "exec", eventlog::writer::Durability::Buffered).unwrap();
    let mut bridge = Bridge::new(w);

    let id = Uuid::new_v4();
    let ev = mk_event(id);

    bridge.publish_once(ev.clone()).unwrap();
    bridge.publish_once(ev.clone()).unwrap();

    // verify persisted log contains exactly 1 event with this id
    let r = EventLogReader::open(&path).unwrap();
    let mut n = 0usize;

    for env in r.iter_envelopes() {
        let env = env.unwrap();
        if env.kind != "event" { continue; }

        let bytes = base64::engine::general_purpose::STANDARD
            .decode(env.payload_b64.as_bytes())
            .unwrap();

        let e: Event = serde_json::from_slice(&bytes).unwrap();
        if e.id == id { n += 1; }
    }

    assert_eq!(n, 1, "expected exactly-once by EventId, got {n}");
}
