use replay::ReplayGuard;
use replay::state::ReplayHealth;

#[test]
fn replay_ignores_until_snapshot() {
    let mut g = ReplayGuard::new();

    assert!(g.allow_event());

    g.on_adapter_signal();
    assert_eq!(g.health, ReplayHealth::NeedSnapshot);
    assert!(!g.allow_event());

    g.on_snapshot();
    assert!(g.allow_event());
}
