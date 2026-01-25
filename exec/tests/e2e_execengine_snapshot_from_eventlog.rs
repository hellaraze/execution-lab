use std::fs;

use anyhow::Result;
use eventlog::reader::EventLogReader;
use eventlog::writer::{Durability, EventLogWriter};

use exec::events::ExecEvent;
use exec::events::OrderId;
use exec::util::instrument::InstrumentKey;

use exec_engine::fsm::OrderEvent;
use exec_engine::store::OrderStore;

fn qty_atoms(x: f64) -> u64 {
    (x * 1_000_000.0).round() as u64
}

#[test]
fn e2e_eventlog_to_exec_engine_snapshot_hash_is_deterministic() -> Result<()> {
    let instrument = InstrumentKey::new("binance", "BTCUSDT");
    let path = std::env::temp_dir().join("replay_e2e_execengine.eventlog");
    let _ = fs::remove_file(&path);

    let id = OrderId(1);

    // minimal realistic stream
    let events: Vec<ExecEvent> = vec![
        ExecEvent::OrderCreated {
            instrument: instrument.clone(),
            id,
        },
        ExecEvent::OrderAcked {
            instrument: instrument.clone(),
            id,
        },
        ExecEvent::OrderFill {
            instrument: instrument.clone(),
            id,
            filled_qty: 0.10,
            avg_px: 100.0,
        },
        ExecEvent::OrderFill {
            instrument: instrument.clone(),
            id,
            filled_qty: 0.20,
            avg_px: 101.0,
        },
    ];

    // write eventlog
    {
        let mut w = EventLogWriter::open_append(&path, "el:replay-e2e", Durability::Buffered)?;
        for ev in &events {
            w.write(ev)?;
        }
    }

    // apply pass #1
    let mut s1 = OrderStore::new();
    s1.get_or_create(1, 10_000_000_000).unwrap();

    {
        let mut r = EventLogReader::open(&path)?;
        while let Some((env, payload)) = r.read_next()? {
            let ev: ExecEvent = serde_json::from_slice(&payload)?;
            match ev {
                ExecEvent::OrderCreated { id, .. } => {
                    let _ = s1.get_or_create(id.0, 10_000_000_000);
                }
                ExecEvent::OrderAcked { id, .. } => {
                    s1.apply(id.0, OrderEvent::Accept).unwrap();
                }
                ExecEvent::OrderFill { id, filled_qty, .. } => {
                    s1.apply(
                        id.0,
                        OrderEvent::Fill {
                            fill_id: env.seq,
                            qty_atoms: qty_atoms(filled_qty),
                        },
                    )
                    .unwrap();
                }
                ExecEvent::OrderCancelled { id, .. } => {
                    let _ = s1.apply(id.0, OrderEvent::Cancel);
                }
                ExecEvent::OrderRejected { id, .. } => {
                    let _ = s1.apply(id.0, OrderEvent::Reject);
                }
                _ => {}
            }
        }
    }

    let h1 = s1.snapshot_hash_hex(1).unwrap();

    // apply pass #2
    let mut s2 = OrderStore::new();
    s2.get_or_create(1, 10_000_000_000).unwrap();

    {
        let mut r = EventLogReader::open(&path)?;
        while let Some((env, payload)) = r.read_next()? {
            let ev: ExecEvent = serde_json::from_slice(&payload)?;
            match ev {
                ExecEvent::OrderCreated { id, .. } => {
                    let _ = s2.get_or_create(id.0, 10_000_000_000);
                }
                ExecEvent::OrderAcked { id, .. } => {
                    s2.apply(id.0, OrderEvent::Accept).unwrap();
                }
                ExecEvent::OrderFill { id, filled_qty, .. } => {
                    s2.apply(
                        id.0,
                        OrderEvent::Fill {
                            fill_id: env.seq,
                            qty_atoms: qty_atoms(filled_qty),
                        },
                    )
                    .unwrap();
                }
                ExecEvent::OrderCancelled { id, .. } => {
                    let _ = s2.apply(id.0, OrderEvent::Cancel);
                }
                ExecEvent::OrderRejected { id, .. } => {
                    let _ = s2.apply(id.0, OrderEvent::Reject);
                }
                _ => {}
            }
        }
    }

    let h2 = s2.snapshot_hash_hex(1).unwrap();
    assert_eq!(h1, h2);

    let _ = fs::remove_file(&path);
    Ok(())
}
