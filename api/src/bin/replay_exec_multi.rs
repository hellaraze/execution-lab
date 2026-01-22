use eventlog::EventLogReader;
use exec::events::ExecEvent;
use exec::order::snapshot::build_snapshot_multi;

fn main() -> anyhow::Result<()> {
    let path = "var/live_exec_multi.log";

    let mut r = EventLogReader::open(path)?;
    let mut events: Vec<ExecEvent> = Vec::new();
    let mut snapshot_hash: Option<u64> = None;

    while let Some((env, payload)) = r.next()? {
        if env.kind == "event" {
            let ev: ExecEvent = serde_json::from_slice(&payload)?;
            events.push(ev);
            continue;
        }

        if env.kind == "snapshot_hash" {
            if payload.len() != 8 {
                anyhow::bail!("snapshot_hash must be 8 bytes, got {}", payload.len());
            }
            let mut b = [0u8; 8];
            b.copy_from_slice(&payload);
            snapshot_hash = Some(u64::from_le_bytes(b));
        }
    }

    let expected = snapshot_hash.ok_or_else(|| anyhow::anyhow!("snapshot_hash missing"))?;
    let (_stores, replay_hash) =
        build_snapshot_multi(&events).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    if expected != replay_hash {
        panic!(
            "MULTI SNAPSHOT HASH MISMATCH: expected={} replay={}",
            expected, replay_hash
        );
    }

    println!("REPLAY MULTI OK. snapshot_hash={}", replay_hash);
    Ok(())
}
