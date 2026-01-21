use adapters::{SeqTracker, AdapterSignal};
use exec::guard::ExecGuard;
use eventlog::snapshot::Snapshot;

/// Replay-driven Live Loop (skeleton).
/// Invariant: live_state_hash == replay_state_hash, иначе PANIC/HALT.
fn main() {
    // Guards
    let mut seq = SeqTracker::new();
    let mut exec_guard = ExecGuard::new();

    // --- LIVE FEED (synthetic for now) ---
    // seq=1 ok
    let _ = seq.observe(1).unwrap();
    assert!(exec_guard.allow_exec());

    // inject GAP: seq jumps to 10 -> NeedSnapshot
    let gap = seq.observe(10);
    assert_eq!(gap, Err(AdapterSignal::NeedSnapshot));
    exec_guard.on_need_snapshot();
    assert!(!exec_guard.allow_exec());

    // --- SNAPSHOT BARRIER ---
    // Snapshot arrives -> unblock execution
    exec_guard.on_snapshot();
    assert!(exec_guard.allow_exec());

    // --- LIVE produces a snapshot state (placeholder u64 state) ---
    let live_state = Snapshot::new(42u64);

    // --- REPLAY recomputes snapshot from event log (placeholder same state) ---
    // Later: this будет реальный replay из eventlog.
    let replay_state = Snapshot::new(42u64);

    // --- ABSOLUTE JUDGE ---
    live_state.assert_same(&replay_state);

    println!("OK: live ≡ replay (snapshot hash match)");
}
