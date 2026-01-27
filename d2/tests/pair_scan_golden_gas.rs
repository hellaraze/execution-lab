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
    // ensure inputs exist (generated elsewhere in workflow)
    assert!(std::path::Path::new("../replay/tests/data/md_a.eventlog").exists());
    assert!(std::path::Path::new("../replay/tests/data/md_b.eventlog").exists());

    // run the bin; must be GAS with shift=150bps
    let obs = "../replay/tests/data/_obs_pair_golden.jsonl";
    let _ = std::fs::remove_file(obs);

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
        "../replay/tests/data/md_a.eventlog",
        "../replay/tests/data/md_b.eventlog",
        "--epsilon",
        "0.0001",
        "--min-edge-bps",
        "1",
        "--b-shift-bps",
        "150",
        "--obs-out",
        obs,
    ]);
    assert_eq!(code, 0, "non-zero exit:\n{out}");
    assert!(out.contains("GAS reason=Pass"), "expected GAS:\n{out}");

    // obs file must exist and contain a RiskEvaluated line with decision=Gas
    let s = std::fs::read_to_string(obs).expect("read obs");
    assert!(
        s.contains("\"RiskEvaluated\""),
        "missing RiskEvaluated:\n{s}"
    );
    assert!(s.contains("decision=Gas"), "missing decision=Gas:\n{s}");

    // timestamp should be present (ts object); we accept nanos=0 for Process,
    // but require the ts fields exist (schema stability)
    assert!(s.contains("\"ts\""), "missing ts:\n{s}");
    assert!(s.contains("\"source\""), "missing ts.source:\n{s}");
}
