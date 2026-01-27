use el_core::time::{TimeSource, Timestamp};
use el_obs::event::ObsEvent;
use el_obs::sink::{FileSink, ObsSink};

#[test]
fn file_sink_writes_jsonl() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("obs.jsonl");

    let mut s = FileSink::open_append(&p).unwrap();
    s.emit(ObsEvent::RiskEvaluated {
        ts: Timestamp::new(0, TimeSource::Exchange),
        verdict: "Allow".to_string(),
    });

    let txt = std::fs::read_to_string(&p).unwrap();
    assert!(txt.contains("\"RiskEvaluated\""));
    assert!(txt.contains("\"verdict\":\"Allow\""));
    assert!(txt.lines().count() == 1);
}
