use std::time::{SystemTime, UNIX_EPOCH};

use eventlog::EventLogWriter;
use exec::events::{ExecEvent, OrderId};
use exec::order::snapshot::build_snapshot_multi;
use exec::util::instrument::InstrumentKey;

fn now_ns() -> u64 {
    let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    d.as_secs() * 1_000_000_000 + d.subsec_nanos() as u64
}

fn main() -> anyhow::Result<()> {
    std::fs::create_dir_all("var")?;
    let path = "var/live_exec_multi.log";
    let _ = std::fs::remove_file(path);

    let mut w = EventLogWriter::open(path)?;
    let btc = InstrumentKey::new("binance", "BTCUSDT");
    let eth = InstrumentKey::new("binance", "ETHUSDT");

    let events = vec![
        ExecEvent::OrderCreated {
            instrument: btc.clone(),
            id: OrderId(1),
        },
        ExecEvent::OrderAcked {
            instrument: btc.clone(),
            id: OrderId(1),
        },
        ExecEvent::OrderCreated {
            instrument: eth.clone(),
            id: OrderId(2),
        },
        ExecEvent::OrderAcked {
            instrument: eth.clone(),
            id: OrderId(2),
        },
    ];

    for ev in &events {
        w.append_bytes("event", now_ns(), &serde_json::to_vec(ev)?)?;
    }

    let (_stores, hash) = build_snapshot_multi(&events)?;
    w.append_bytes("snapshot_hash", now_ns(), &hash.to_le_bytes())?;
    w.flush()?;

    println!("LIVE MULTI OK. snapshot_hash={}", hash);
    Ok(())
}
