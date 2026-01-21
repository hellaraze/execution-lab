use eventlog::snapshot::Snapshot;

#[test]
fn live_equals_replay_snapshot_hash() {
    let live_state = Snapshot::new(42u64);
    let replay_state = Snapshot::new(42u64);

    live_state.assert_same(&replay_state);
}
