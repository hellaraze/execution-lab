use std::path::PathBuf;
use std::process::Command;

fn run(args: &[&str]) -> (i32, String) {
    let out = Command::new("cargo").args(args).output().expect("spawn");
    let code = out.status.code().unwrap_or(1);
    let s =
        String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
    (code, s)
}

#[test]
fn d2_pair_scan_emits_gas_deterministically_with_shift() {
    // Use a COMMITTED fixture (CI-safe). We pass the same file for A and B and
    // rely on --b-shift-bps to synthesize deterministic GAS.
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf();

    let base = root.join("replay/tests/data/binance_depth_fixture.eventlog");
    assert!(base.exists(), "missing BASE fixture: {}", base.display());

    let obs = std::env::temp_dir().join("execution_lab_obs_pair_golden.jsonl");
    let _ = std::fs::remove_file(&obs);

    let base_s = base.to_string_lossy().into_owned();
    let obs_s = obs.to_string_lossy().into_owned();

    let args: Vec<&str> = vec![
        "run",
        "-q",
        "-p",
        "d2",
        "--features",
        "replay-ro",
        "--bin",
        "d2_pair_scan",
        "--",
        base_s.as_str(),
        base_s.as_str(),
        "--epsilon",
        "0.0001",
        "--min-edge-bps",
        "1",
        "--b-shift-bps",
        "150",
        "--obs-out",
        obs_s.as_str(),
    ];

    let (code, out) = run(&args);
    assert_eq!(code, 0, "non-zero exit:\n{out}");
    assert!(out.contains("GAS reason=Pass"), "expected GAS:\n{out}");

    let s = std::fs::read_to_string(&obs).expect("read obs");
    assert!(
        s.contains("\"RiskEvaluated\""),
        "missing RiskEvaluated:\n{s}"
    );
    assert!(s.contains("decision=Gas"), "missing decision=Gas:\n{s}");
    assert!(s.contains("\"ts\""), "missing ts:\n{s}");
    assert!(s.contains("\"source\""), "missing ts.source:\n{s}");
}
