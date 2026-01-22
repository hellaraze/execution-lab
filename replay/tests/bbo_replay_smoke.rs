#[test]
fn bbo_replay_not_empty() {
    let log_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../var/events.eventlog");

    let out = std::process::Command::new("cargo")
        .args(["run", "-q", "-p", "replay", "--bin", "replay", "--", log_path])
        .output()
        .expect("run replay");

    assert!(out.status.success(), "replay failed: {}", String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);

    // Expect FINAL ... bid=Some ... ask=Some
    assert!(stdout.contains("FINAL"), "no FINAL line in stdout: {}", stdout);
    assert!(stdout.contains("bid=Some("), "bid is empty: {}", stdout);
    assert!(stdout.contains("ask=Some("), "ask is empty: {}", stdout);
}
