use exec::guard::ExecGuard;

#[test]
fn exec_is_blocked_until_snapshot() {
    let mut g = ExecGuard::new();

    assert!(g.allow_exec());

    g.on_need_snapshot();
    assert!(!g.allow_exec());

    g.on_snapshot();
    assert!(g.allow_exec());
}
