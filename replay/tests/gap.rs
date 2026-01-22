use anyhow::Result;
use el_core::event::{Event, EventPayload, EventType, Exchange};
use el_core::time::{Timestamp, TimeSource};
use eventlog::EventLogWriter;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

mod util;

fn ts(n: i64) -> Timestamp {
    Timestamp::new(n, TimeSource::Process)
}

fn mk_delta(i: u64) -> Event {
    Event {
        id: uuid::Uuid::new_v4(),
        event_type: EventType::BookDelta,
        instrument: el_core::instrument::InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
        exchange: Exchange::Binance,
        symbol: "BTCUSDT".to_string(),
        ts_exchange: None,
        ts_recv: ts(i as i64),
        ts_proc: ts(i as i64),
        seq: Some(i),
        schema_version: 1,
        integrity_flags: vec![],
        payload: EventPayload::BookDelta {
            bids: vec![(50_000.0 + (i % 10) as f64, 1.0)],
            asks: vec![],
        },
        meta: HashMap::new(),
    }
}

#[test]
fn gap_is_detected_and_fails_fast() -> Result<()> {
    let mut dir = PathBuf::from("replay/target/tmp");
    fs::create_dir_all(&dir)?;
    dir.push("gap_test.log");

    // 1) write a clean log
    {
        let mut w = EventLogWriter::open(&dir)?;
        for i in 1..=80u64 {
            w.write(&mk_delta(i))?;
        }
        w.flush()?;
    }

    // 2) corrupt it: remove one envelope line (create seq gap)
    let s = fs::read_to_string(&dir)?;
    let mut lines: Vec<&str> = s.lines().collect();
    // remove line 40 (1-indexed) => gap
    lines.remove(39);
    let corrupted = lines.join("
") + "
";
    fs::write(&dir, corrupted)?;

    // 3) run replay: must fail with GAP
    let err = util::run_and_collect_chain_hashes(dir.to_str().unwrap(), 80).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("GAP DETECTED"),
        "expected GAP DETECTED, got: {}",
        msg
    );

    Ok(())
}
