use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use uuid::Uuid;

use el_core::event::{Event, EventPayload, EventType, Exchange};
use el_core::time::{Timestamp, TimeSource};

use exec::order::bridge::to_exec_event;
use exec::order::snapshot::build_snapshot;
use exec::events::ExecEvent;

use eventlog::EventLogWriter;

fn now_ns_i64() -> i64 {
    let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    (d.as_secs() * 1_000_000_000 + d.subsec_nanos() as u64) as i64
}

fn now_ns_u64() -> u64 {
    let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    d.as_secs() * 1_000_000_000 + d.subsec_nanos() as u64
}

fn mk_event(event_type: EventType, payload: EventPayload) -> Event {
    let t = now_ns_i64();
    Event {
        id: Uuid::new_v4(),
        event_type,
        exchange: Exchange::Binance,
        symbol: "BTCUSDT".to_string(),
        instrument: el_core::instrument::InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
        ts_exchange: None,
        ts_recv: Timestamp::new(t, TimeSource::Receive),
        ts_proc: Timestamp::new(t, TimeSource::Process),
        seq: None,
        schema_version: 1,
        integrity_flags: vec![],
        payload,
        meta: HashMap::new(),
    }
}

fn main() -> anyhow::Result<()> {
    let path = "var/e2e_core_exec.log";
    std::fs::create_dir_all("var")?;
    let _ = std::fs::remove_file(path);

    let w = EventLogWriter::open(path)?;
    let mut outbox = Bridge::new(w);

    let order_id = "ORD-1".to_string();

    let core_events = vec![
        mk_event(
            EventType::OrderSubmit,
            EventPayload::OrderSubmit {
                order_id: order_id.clone(),
                side: "buy".to_string(),
                price: 100.0,
                qty: 1.0,
            },
        ),
        mk_event(
            EventType::OrderAck,
            EventPayload::OrderAck {
                order_id: order_id.clone(),
            },
        ),
        mk_event(
            EventType::Fill,
            EventPayload::Fill {
                order_id: order_id.clone(),
                fill_id: "F1".to_string(),
                price: 100.0,
                qty: 1.0,
            },
        ),
        mk_event(
            EventType::CancelRequest,
            EventPayload::CancelRequest {
                order_id: order_id.clone(),
            },
        ),
        mk_event(
            EventType::CancelAck,
            EventPayload::CancelAck {
                order_id: order_id.clone(),
            },
        ),
    ];

    let mut exec_events: Vec<ExecEvent> = Vec::new();
    for ev in &core_events {
        if let Some(x) = to_exec_event(ev)? {
            outbox.publish_once(x.clone())?;
            exec_events.push(x);
        }
    }

    let (_store, h) = build_snapshot(&exec_events).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    w.append_bytes("snapshot_hash", now_ns_u64(), &h.to_le_bytes())?;
    w.flush()?;

    println!("E2E OK: wrote exec events + snapshot_hash={}", h);
    Ok(())
}
