use std::time::{SystemTime, UNIX_EPOCH};

use eventlog::{EventLogReader, EventLogWriter};
use eventlog::hash::stable_hash;
use eventlog::snapshot::Snapshot;

fn now_ns() -> u64 {
    let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    d.as_secs() * 1_000_000_000 + d.subsec_nanos() as u64
}

fn reduce(mut state: u64, payload: &[u8]) -> u64 {
    let h = stable_hash(&payload.to_vec());
    state = state.wrapping_mul(1_000_000_003) ^ h;
    state
}

fn main() -> anyhow::Result<()> {
    let path = "var/live_real_mismatch.log";
    std::fs::create_dir_all("var")?;
    let _ = std::fs::remove_file(path);

    // LIVE: пишем 1 event
    let mut w = EventLogWriter::open(path)?;
    let mut live_state: u64 = 0;

    let p = br#"{"ev":"x","v":9}"#;
    w.append_bytes("event", now_ns(), p)?;
    live_state = reduce(live_state, p);
    w.flush()?;

    // REPLAY: читаем, но специально делаем другой стартовый state
    let mut r = EventLogReader::open(path)?;
    let mut replay_state: u64 = 123;

    while let Some((env, payload)) = r.next()? {
        if env.kind == "event" {
            replay_state = reduce(replay_state, &payload);
        }
    }

    let live_snap = Snapshot::new(live_state);
    let replay_snap = Snapshot::new(replay_state);

    // Должно паникнуть
    live_snap.assert_same(&replay_snap);

    Ok(())
}
