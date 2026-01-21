use eventlog::snapshot::Snapshot;

fn main() {
    let live_state = Snapshot::new(42u64);
    let replay_state = Snapshot::new(43u64);
    live_state.assert_same(&replay_state);
}
