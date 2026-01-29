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
    const MD_A: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../replay/tests/data/md_a.eventlog"
    );
    const MD_B: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../replay/tests/data/md_b.eventlog"
    );
    const OBS: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../replay/tests/data/_obs_pair_golden.jsonl"
    );

    // ensure inputs exist (cwd-independent)
    assert!(std::path::Path::new(MD_A).exists(), "missing MD_A: {MD_A}");
    assert!(std::path::Path::new(MD_B).exists(), "missing MD_B: {MD_B}");

    // obs output (best-effort cleanup)
    let _ = std::fs::remove_file(OBS);

    // run the bin; must be GAS with shift=150bps
    let (code, out) = run(&[
        "run",
        "-q",
        "-p",
        "d2",
        "--features",
        "replay-ro",
        "--bin",
        "d2_pair_scan",
        "--",
        MD_A,
        MD_B,
        "--epsilon",
        "0.0001",
        "--min-edge-bps",
        "1",
        "--b-shift-bps",
        "150",
        "--obs-out",
        OBS,
    ]);
    assert_eq!(code, 0, "non-zero exit:\n{out}");
    assert!(out.contains("GAS reason=Pass"), "expected GAS:\n{out}");

    // obs file must exist and contain a RiskEvaluated line with decision=Gas
    let s = std::fs::read_to_string(OBS).expect("read obs");
    assert!(
        s.contains("\"RiskEvaluated\""),
        "missing RiskEvaluated:\n{s}"
    );
    assert!(s.contains("decision=Gas"), "missing decision=Gas:\n{s}");

    // timestamp contract
    assert!(s.contains("\"ts\""), "missing ts:\n{s}");
    assert!(s.contains("\"source\""), "missing ts.source:\n{s}");
}
