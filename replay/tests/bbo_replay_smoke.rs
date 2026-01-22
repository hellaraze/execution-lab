use std::fs;
use std::process::Command;

#[test]
fn bbo_replay_not_empty() {
    // take a small sample from repo-root events.log (raw core::Event lines; TickerBbo)
    let in_root = concat!(env!("CARGO_MANIFEST_DIR"), "/../events.log");
    let tmp_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data");
    let tmp_raw = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/_tmp_bbo_raw.log");
    let tmp_evlog = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/_tmp_bbo.eventlog");

    fs::create_dir_all(tmp_dir).unwrap();
    let _ = fs::remove_file(tmp_raw);
    let _ = fs::remove_file(tmp_evlog);

    // copy first ~50 lines (fast, deterministic enough)
    let s = fs::read_to_string(in_root).expect("read events.log");
    let mut out = String::new();
    for (i, line) in s.lines().take(50).enumerate() {
        if !line.trim().is_empty() {
            out.push_str(line);
            out.push('\n');
        }
        if i >= 49 {
            break;
        }
    }
    fs::write(tmp_raw, out).expect("write tmp raw");

    // convert raw -> eventlog
    let st = Command::new("cargo")
        .args([
            "run",
            "-q",
            "-p",
            "replay",
            "--bin",
            "raw_core_to_eventlog",
            "--",
            tmp_raw,
            tmp_evlog,
        ])
        .status()
        .expect("run raw_core_to_eventlog");
    assert!(st.success(), "raw_core_to_eventlog failed");

    // replay should materialize bid/ask from TickerBbo
    let out = Command::new("cargo")
        .args([
            "run", "-q", "-p", "replay", "--bin", "replay", "--", tmp_evlog,
        ])
        .output()
        .expect("run replay");
    assert!(
        out.status.success(),
        "replay failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("bid=Some("),
        "expected bid not empty; got: {stdout}"
    );
    assert!(
        stdout.contains("ask=Some("),
        "expected ask not empty; got: {stdout}"
    );

    let _ = fs::remove_file(tmp_raw);
    let _ = fs::remove_file(tmp_evlog);
}
