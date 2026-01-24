use std::fs;

use exec::util::instrument::InstrumentKey;


use anyhow::Result;
use eventlog::writer::{Durability, EventLogWriter};
use eventlog::reader::EventLogReader;

use exec::events::{ExecEvent, OrderId};

use exec_engine::store::OrderStore;
use exec_engine::fsm::OrderEvent;

fn qty_atoms(x: f64) -> u64 {
    // deterministic scaling (micro-atoms)
    (x * 1_000_000.0).round() as u64
}

#[test]
fn e2e_eventlog_to_exec_engine_snapshot_hash_is_deterministic() -> Result<()> {
    // temp file
    let path = std::env::temp_dir().join("exec_e2e_execengine.eventlog");
    let _ = fs::remove_file(&path);

    // build a minimal, realistic exec event stream (same style as exec/tests/eventlog_roundtrip.rs)
    // NOTE: we intentionally do NOT depend on fill_id from exchange; we use eventlog seq as fill_id.
    let instrument = {
        // reuse whatever exec crate already uses for tests; this should compile in your workspace
        // (the type is whatever ExecEvent expects)
        InstrumentKey::new("Binance", "BTCUSDT")
    };

    let id = OrderId(1);

    let events: Vec<ExecEvent> = vec![
        ExecEvent::OrderCreated { instrument: instrument.clone(), id },
        ExecEvent::OrderAcked   { instrument: instrument.clone(), id },
        ExecEvent::OrderFill    { instrument: instrument.clone(), id, filled_qty: 0.10, avg_px: 100.0 },
        ExecEvent::OrderFill    { instrument: instrument.clone(), id, filled_qty: 0.20, avg_px: 101.0 },
    ];

    // write to eventlog
    {
        let mut w = EventLogWriter::open_append(&path, "el:exec-e2e", Durability::Buffered)?;
        for ev in &events {
            // writer.write() uses kind="event", ts=0
            w.write(ev)?;
        }
    }

    // read back (with seq) and apply into exec_engine
    let mut store1 = OrderStore::new();
    store1.get_or_create(1, 10_000_000_000).unwrap(); // big total_atoms (we only test idempotency/hash)

    {
        let mut r = EventLogReader::open(&path)?;
        while let Some((env, payload)) = r.next()? {
            let ev: ExecEvent = serde_json::from_slice(&payload)?;
            match ev {
                ExecEvent::OrderCreated { id, .. } => {
                    let _ = store1.get_or_create(id.0, 10_000_000_000);
                }
                ExecEvent::OrderAcked { id, .. } => {
                    store1.apply(id.0, OrderEvent::Accept).unwrap();
                }
                ExecEvent::OrderFill { id, filled_qty, .. } => {
                    store1.apply(
                        id.0,
                        OrderEvent::Fill { fill_id: env.seq as u64, qty_atoms: qty_atoms(filled_qty) },
                    ).unwrap();
                }
                ExecEvent::OrderCancelled { id, .. } => {
                    let _ = store1.apply(id.0, OrderEvent::Cancel);
                }
                ExecEvent::OrderRejected { id, .. } => {
                    let _ = store1.apply(id.0, OrderEvent::Reject);
                }
                _ => { /* ignore */ }
            }
        }
    }

    let h1 = store1.snapshot_hash_hex(1).unwrap();

    // determinism check: re-read and apply into fresh store -> same hash
    let mut store2 = OrderStore::new();
    store2.get_or_create(1, 10_000_000_000).unwrap();

    {
        let mut r = EventLogReader::open(&path)?;
        while let Some((env, payload)) = r.next()? {
            let ev: ExecEvent = serde_json::from_slice(&payload)?;
            match ev {
                ExecEvent::OrderCreated { id, .. } => {
                    let _ = store2.get_or_create(id.0, 10_000_000_000);
                }
                ExecEvent::OrderAcked { id, .. } => {
                    store2.apply(id.0, OrderEvent::Accept).unwrap();
                }
                ExecEvent::OrderFill { id, filled_qty, .. } => {
                    store2.apply(
                        id.0,
                        OrderEvent::Fill { fill_id: env.seq as u64, qty_atoms: qty_atoms(filled_qty) },
                    ).unwrap();
                }
                ExecEvent::OrderCancelled { id, .. } => {
                    let _ = store2.apply(id.0, OrderEvent::Cancel);
                }
                ExecEvent::OrderRejected { id, .. } => {
                    let _ = store2.apply(id.0, OrderEvent::Reject);
                }
                _ => {}
            }
        }
    }

    let h2 = store2.snapshot_hash_hex(1).unwrap();
    assert_eq!(h1, h2);

    // cleanup
    let _ = fs::remove_file(&path);

    Ok(())
}
