use elctl::config;

#[test]
fn config_v1_parses() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(tmp.path(), "version = 1\nmode = \"demo\"\n").unwrap();

    let cfg = config::load(tmp.path().to_str().unwrap()).unwrap();
    assert_eq!(cfg.version, 1);
}
