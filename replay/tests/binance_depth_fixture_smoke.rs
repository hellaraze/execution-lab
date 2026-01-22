use std::process::Command;

#[test]
fn binance_depth_fixture_replay_not_empty() {
    let log_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/binance_depth_fixture.eventlog");

    let out = Command::new("cargo")
        .args(["run", "-q", "-p", "replay", "--bin", "replay", "--", log_path])
        .output()
        .expect("run replay");

    assert!(out.status.success(), "replay failed: {}", String::from_utf8_lossy(&out.stderr));
    let s = String::from_utf8_lossy(&out.stdout);

    // ждём что replay реально применил что-то: bid/ask должны быть Some(
    assert!(s.contains("bid=Some("), "expected bid Some, got:\n{s}");
    assert!(s.contains("ask=Some("), "expected ask Some, got:\n{s}");

    // и что есть deltas, а не только один snapshot
    assert!(s.contains("delta="), "expected counters, got:\n{s}");
    assert!(!s.contains("delta=0"), "expected delta>0, got:\n{s}");
}
