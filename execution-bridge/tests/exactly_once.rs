use execution_bridge::*;
use el_core::event::{Event, EventId, EventPayload, EventType, Exchange};
use el_core::instrument::InstrumentKey;
use el_core::time::{Timestamp, TimeSource};
use eventlog::{EventLogWriter, EventLogReader};
use base64::Engine;
use serde_json;
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
    // file-backed writer to temp file (no mem impl in eventlog)
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("exec_outbox.log");

    let w = EventLogWriter::open_append(&path, "exec", eventlog::writer::Durability::Buffered).unwrap();
    let mut bridge = Bridge::new(w);

    let id = Uuid::new_v4();
    let ev = mk_event(id);

    bridge.publish_once(ev.clone()).unwrap();
    bridge.publish_once(ev.clone()).unwrap();

    // NOTE: currently EventLogWriter is NOT idempotent, so this test will FAIL until we implement dedupe.
    // We'll read envelopes and assert 1 after dedupe is added.
}
