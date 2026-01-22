use std::fs;

#[test]
fn exec_events_hash_golden() {
    let log_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/exec_events.log");
    let out = std::process::Command::new("cargo")
        .args([
            "run",
            "-q",
            "-p",
            "replay",
            "--bin",
            "exec_hash",
            "--",
            log_path,
        ])
        .output()
        .expect("run exec_hash");
    assert!(
        out.status.success(),
        "exec_hash failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    // parse "... hash=NUM"
    let hash = stdout
        .split("hash=")
        .nth(1)
        .and_then(|s| s.split_whitespace().next())
        .expect("hash=... in output");
    let hash_u: u64 = hash.parse().expect("hash is u64");

    let golden = fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/exec_events_hash.txt"
    ))
    .expect("read golden file");
    let expected = golden
        .lines()
        .find_map(|l| l.strip_prefix("EXPECTED_HASH="))
        .expect("EXPECTED_HASH=... in golden file");
    let expected_u: u64 = expected.parse().expect("expected hash u64");

    assert_eq!(hash_u, expected_u, "stdout was: {}", stdout);
}
