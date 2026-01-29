use std::path::Path;

#[test]
fn config_v1_replay_parses_and_validates() {
    let dir = tempfile::tempdir().unwrap();
    let ev = dir.path().join("fixture.eventlog");
    std::fs::write(&ev, b"{}").unwrap();

    let cfg_path = dir.path().join("replay.toml");
    std::fs::write(
        &cfg_path,
        format!(
            r#"
config_version = 1
mode = "replay"
eventlog = "{}"

[paths]
state_dir = "state"
runs_dir = "runs"
"#,
            ev.display()
        ),
    )
    .unwrap();

    let (cfg, _raw) = el_runtime::product::ProductConfigV1::load(Path::new(&cfg_path)).unwrap();
    cfg.validate().unwrap();
}
