use eventlog::EventLogReader;
use exec::events::{ExecEvent, OrderId};
use exec::order::snapshot::build_snapshot;
use replay::ReplayGuard;

fn main() -> anyhow::Result<()> {
    let path = "var/live_exec.log";

    let mut r = EventLogReader::open(path)?;
    let mut events: Vec<ExecEvent> = Vec::new();

    let mut guard = ReplayGuard::new();
    let mut snap_hash: Option<u64> = None;

    while let Some((env, payload)) = r.read_next()? {
        guard.on_kind(&env.kind);

        if env.kind == "snapshot_hash" {
            let mut b = [0u8; 8];
            b.copy_from_slice(&payload);
            snap_hash = Some(u64::from_le_bytes(b));
            continue;
        }

        if env.kind == "snapshot" {
            continue;
        }

        if !guard.allow_event() {
            continue;
        }

        if env.kind == "event" {
            let ev: ExecEvent = serde_json::from_slice(&payload)?;
            events.push(ev);
        }
    }

    // ЛОМАЕМ replay: добавляем лишнее событие
    events.push(ExecEvent::OrderRejected {
        instrument: exec::util::instrument::InstrumentKey::new("binance", "BTCUSDT"),
        id: OrderId(1),
        reason: "boom".to_string(),
    });

    let (_store, replay_hash) =
        build_snapshot(&events).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let snap_hash = snap_hash.expect("snapshot_hash must exist");

    if snap_hash != replay_hash {
        panic!(
            "EXEC HASH MISMATCH: snapshot_hash={} replay={}",
            snap_hash, replay_hash
        );
    }

    Ok(())
}
