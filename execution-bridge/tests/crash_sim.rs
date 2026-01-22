use execution_bridge::{Bridge, ExecOutbox};
use el_core::event::Event;
use tempfile::tempdir;
use uuid::Uuid;

fn mk_event(id: Uuid) -> Event {
    Event {
        id,
        event_type: el_core::event::EventType::Custom,
        exchange: el_core::event::Exchange::Binance,
        symbol: "TEST".into(),
        instrument: el_core::instrument::InstrumentKey::new(
            el_core::event::Exchange::Binance,
            "TEST".into()
        ),
        ts_exchange: None,
        ts_recv: el_core::time::Timestamp::now(el_core::time::TimeSource::Receive),
        ts_proc: el_core::time::Timestamp::now(el_core::time::TimeSource::Process),
        seq: None,
        schema_version: 1,
        integrity_flags: vec![],
        payload: el_core::event::EventPayload::Custom,
        meta: Default::default(),
    }
}

#[test]
fn crash_restart_no_duplicates() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("exec.log");

    let id = Uuid::new_v4();

    {
        let mut b = Bridge::open_dedup(
            &path,
            "exec",
            eventlog::writer::Durability::FsyncEvery { n: 1 },
        ).unwrap();
        b.publish_once(mk_event(id)).unwrap();
    }

    {
        let mut b = Bridge::open_dedup(
            &path,
            "exec",
            eventlog::writer::Durability::FsyncEvery { n: 1 },
        ).unwrap();
        b.publish_once(mk_event(id)).unwrap();
    }

    let mut r = eventlog::EventLogReader::open(&path).unwrap();
    let mut n = 0;
    while let Some((env, payload)) = r.next().unwrap() {
        if env.kind != "event" { continue; }
        let e: Event = serde_json::from_slice(&payload).unwrap();
        if e.id == id { n += 1; }
    }

    assert_eq!(n, 1);
}
