use std::process::Command;

#[test]
fn raw_to_eventlog_smoke() {
    // input is repo-root events_book.log (legacy raw json line)
    let in_path = "events_book.log";
    let out_path = "replay/tests/data/_tmp_events_book.eventlog";

    // run converter
    let status = Command::new("cargo")
        .args([
            "run",
            "-q",
            "-p",
            "replay",
            "--bin",
            "raw_to_eventlog",
            "--",
            in_path,
            out_path,
        ])
        .status()
        .expect("run raw_to_eventlog");
    assert!(status.success(), "raw_to_eventlog failed");

    // run replay over produced eventlog
    let status = Command::new("cargo")
        .args([
            "run",
            "-q",
            "-p",
            "replay",
            "--bin",
            "replay",
            "--",
            out_path,
        ])
        .status()
        .expect("run replay");
    assert!(status.success(), "replay failed on converted eventlog");

    let _ = std::fs::remove_file(out_path);
}
