use std::path::PathBuf;
use std::process::Command;

#[test]
fn md_replay_drift_fixture_smoke_and_percentiles_present() {
    let exe = env!("CARGO_BIN_EXE_md_replay");

    // Make path absolute so test doesn't depend on current working directory.
    let mut fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fixture.push("tests/fixtures/binance_md_fixture.eventlog");

    assert!(
        fixture.exists(),
        "fixture missing at {:?} (cwd={:?})",
        fixture,
        std::env::current_dir().ok()
    );

    let out = Command::new(exe)
        .args([
            fixture.to_string_lossy().as_ref(),
            "--mode",
            "compare",
            "--tick",
            "0.01",
            "--window-ms",
            "250",
            "--ring",
            "4096",
            "--max-print",
            "0",
            "--drift-keep",
            "200000",
        ])
        .output()
        .expect("run md_replay");

    assert!(
        out.status.success(),
        "md_replay failed: status={:?}\nstdout:\n{}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let s = String::from_utf8_lossy(&out.stdout);
    let ok = s.lines().find(|l| l.starts_with("OK md_replay ")).expect("OK line");

    assert!(ok.contains("mode=Compare"), "bad ok line: {ok}");
    assert!(ok.contains("bbo="), "missing bbo: {ok}");
    assert!(ok.contains("compared="), "missing compared: {ok}");
    assert!(!ok.contains("drift_ticks_p90=None"), "p90 None: {ok}");
    assert!(!ok.contains("drift_ticks_p99=None"), "p99 None: {ok}");
}
