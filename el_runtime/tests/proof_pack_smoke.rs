#[test]
fn proof_pack_creates_tar_gz() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    let runs_dir = root.join("runs");
    let state_dir = root.join("state");
    let ev = root.join("fixture.eventlog");
    std::fs::write(&ev, b"{}").unwrap();

    let cfg_path = root.join("cfg.toml");
    std::fs::write(
        &cfg_path,
        format!(
            r#"
config_version = 1
mode = "replay"
eventlog = "{}"

[paths]
state_dir = "{}"
runs_dir = "{}"
"#,
            ev.display(),
            state_dir.display(),
            runs_dir.display(),
        ),
    )
    .unwrap();

    let (_id, run_dir) = el_runtime::product::run_from_config_path(&cfg_path).unwrap();
    let pack = el_runtime::proof_pack::create_proof_pack(&run_dir).unwrap();

    assert!(pack.exists());
    assert!(run_dir.join("proof").join("toolchain.txt").exists());
    assert!(run_dir.join("proof").join("env.txt").exists());
    assert!(run_dir.join("proof").join("proof_manifest.json").exists());
}
