use eventlog::EventLogReader;
use exec::events::ExecEvent;
use exec::order::snapshot::build_snapshot;
use replay::ReplayGuard;

fn main() -> anyhow::Result<()> {
    let path = "var/live_exec.log";

    let mut r = EventLogReader::open(path)?;
    let mut events: Vec<ExecEvent> = Vec::new();

    let mut guard = ReplayGuard::new();
    let mut last_snapshot_hash: Option<u64> = None;

    while let Some((env, payload)) = r.next()? {
        // snapshot barrier unblocks replay processing
        guard.on_kind(&env.kind);

        if env.kind == "snapshot_hash" {
            if payload.len() != 8 {
                anyhow::bail!("snapshot_hash payload must be 8 bytes u64, got {}", payload.len());
            }
            let mut b = [0u8; 8];
            b.copy_from_slice(&payload);
            last_snapshot_hash = Some(u64::from_le_bytes(b));
            continue;
        }

        // plain snapshot barrier: ignore payload
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

    let (_store, replay_hash) = build_snapshot(&events).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    println!("REPLAY EXEC OK. replay_hash={}", replay_hash);

    let snap_hash = last_snapshot_hash.ok_or_else(|| anyhow::anyhow!("no snapshot_hash found in log"))?;
    if snap_hash != replay_hash {
        panic!("EXEC HASH MISMATCH: snapshot_hash={} replay={}", snap_hash, replay_hash);
    }

    println!("OK: snapshot_hash == replay_hash (halt on mismatch enabled)");
    Ok(())
}
