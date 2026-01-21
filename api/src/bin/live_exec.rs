use std::time::{SystemTime, UNIX_EPOCH};

use adapters::{SeqTracker, AdapterSignal};
use eventlog::EventLogWriter;
use exec::events::{ExecEvent, OrderId};
use exec::order::snapshot::build_snapshot;
use exec::util::instrument::InstrumentKey;
use replay::ReplayGuard;

fn now_ns() -> u64 {
    let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    d.as_secs() * 1_000_000_000 + d.subsec_nanos() as u64
}

fn main() -> anyhow::Result<()> {
    let path = "var/live_exec.log";
    std::fs::create_dir_all("var")?;
    let _ = std::fs::remove_file(path);

    let mut w = EventLogWriter::open(path)?;

    let mut seq = SeqTracker::new();
    let mut guard = ReplayGuard::new();

    let instrument = InstrumentKey::new("binance", "BTCUSDT");

    let mut live_events: Vec<ExecEvent> = Vec::new();

    // --- event #1 (ok) ---
    seq.observe(1).unwrap();
    if guard.allow_event() {
        let ev = ExecEvent::OrderCreated { instrument: instrument.clone(), id: OrderId(1) };
        w.append_bytes("event", now_ns(), &serde_json::to_vec(&ev)?)?;
        live_events.push(ev);
    }

    // --- GAP -> NeedSnapshot (block) ---
    let gap = seq.observe(10);
    assert_eq!(gap, Err(AdapterSignal::NeedSnapshot));
    guard.on_adapter_signal();

    // --- SNAPSHOT BARRIER (unblock + reset seq) ---
    w.append_bytes("snapshot", now_ns(), &0u64.to_le_bytes())?;
    guard.on_snapshot();
    seq.reset(10);

    // --- event #2 after snapshot ---
    seq.observe(11).unwrap();
    if guard.allow_event() {
        let ev = ExecEvent::OrderAcked { instrument: instrument.clone(), id: OrderId(1) };
        w.append_bytes("event", now_ns(), &serde_json::to_vec(&ev)?)?;
        live_events.push(ev);
    }

    // --- FINAL COMMIT SNAPSHOT (hash) ---
    let (_store, live_hash) = build_snapshot(&live_events).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    w.append_bytes("snapshot_hash", now_ns(), &live_hash.to_le_bytes())?;

    w.flush()?;

    println!("LIVE EXEC OK. live_hash={}", live_hash);
    Ok(())
}
