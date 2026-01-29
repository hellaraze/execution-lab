#[test]
fn run_replay_creates_run_artifacts() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    let runs_dir = root.join("runs");
    let state_dir = root.join("state");

    // create a minimal eventlog file so validation passes
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

    assert!(run_dir.exists());
    assert!(run_dir.join("run_manifest.json").exists());
    assert!(run_dir.join("config.toml").exists());
    assert!(run_dir.join("decisions.log").exists());
    assert!(run_dir.join("sha256.txt").exists());
    assert!(run_dir.join("replay_input_sha256.txt").exists());
    assert!(run_dir.join("RUN_OK").exists());
}
