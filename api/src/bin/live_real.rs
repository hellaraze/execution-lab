use std::time::{SystemTime, UNIX_EPOCH};

use adapters::{SeqTracker, AdapterSignal};
use eventlog::{EventLogReader, EventLogWriter};
use eventlog::hash::stable_hash;
use eventlog::snapshot::Snapshot;
use replay::ReplayGuard;

fn now_ns() -> u64 {
    let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    d.as_secs() * 1_000_000_000 + d.subsec_nanos() as u64
}

// Дет. редьюсер состояния (placeholder reducer).
// Важно: одинаково для live и replay.
fn reduce(mut state: u64, payload: &[u8]) -> u64 {
    let h = stable_hash(&payload.to_vec());
    state = state.wrapping_mul(1_000_000_003) ^ h;
    state
}

fn main() -> anyhow::Result<()> {
    // log path локально в проекте
    let path = "var/live_real.log";
    std::fs::create_dir_all("var")?;
    // стартуем с чистого файла
    let _ = std::fs::remove_file(path);

    // --- LIVE ---
    let mut w = EventLogWriter::open(path)?;
    let mut seq = SeqTracker::new();
    let mut guard = ReplayGuard::new();

    let mut live_state: u64 = 0;

    // event #1 OK
    seq.observe(1).unwrap();
    if guard.allow_event() {
        let p = br#"{"ev":"a","v":1}"#;
        w.append_bytes("event", now_ns(), p)?;
        live_state = reduce(live_state, p);
    }

    // GAP: seq jump -> NeedSnapshot -> block
    let gap = seq.observe(10);
    assert_eq!(gap, Err(AdapterSignal::NeedSnapshot));
    guard.on_adapter_signal();
    // в блоке мы события НЕ применяем (но для демонстрации можем "получить" их и проигнорить)

    // SNAPSHOT BARRIER: пишем snapshot payload = live_state (как bytes)
    // и UNBLOCK + reset seq tracker
    let snap_bytes = live_state.to_le_bytes();
    w.append_bytes("snapshot", now_ns(), &snap_bytes)?;
    guard.on_snapshot();
    seq.reset(10);

    // event #2 после snapshot
    seq.observe(11).unwrap();
    if guard.allow_event() {
        let p = br#"{"ev":"b","v":2}"#;
        w.append_bytes("event", now_ns(), p)?;
        live_state = reduce(live_state, p);
    }

    w.flush()?;

    let live_snap = Snapshot::new(live_state);

    // --- REPLAY ---
    let mut r = EventLogReader::open(path)?;
    let mut replay_guard = ReplayGuard::new();
    let mut replay_state: u64 = 0;

    while let Some((env, payload)) = r.next()? {
        replay_guard.on_kind(&env.kind);

        if env.kind == "snapshot" {
            // snapshot барьер: в нашем контракте это "подтверждение истины"
            // (можно было бы восстановить state из payload, но мы держим reducer как абсолютную функцию от событий)
            continue;
        }

        if !replay_guard.allow_event() {
            continue;
        }

        if env.kind == "event" {
            replay_state = reduce(replay_state, &payload);
        }
    }

    let replay_snap = Snapshot::new(replay_state);

    // --- ABSOLUTE JUDGE ---
    live_snap.assert_same(&replay_snap);

    println!("OK REAL: live ≡ replay (eventlog-backed)");
    Ok(())
}
