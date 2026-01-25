use std::fs;

use anyhow::Result;
use eventlog::reader::EventLogReader;

use exec::events::ExecEvent;
use exec::order::snapshot::build_snapshot;

fn main_hash(events: &[ExecEvent]) -> Result<u64> {
    let (_store, h) = build_snapshot(events).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(h)
}

#[test]
fn exec_fixture_snapshot_hash_golden() -> Result<()> {
    let fixture_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/exec_fixture.eventlog");
    let mut r = EventLogReader::open(fixture_path)?;

    let mut events: Vec<ExecEvent> = Vec::new();
    while let Some((_env, payload)) = r.read_next()? {
        let ev: ExecEvent = serde_json::from_slice(&payload)?;
        events.push(ev);
    }

    let h = main_hash(&events)?;
    let hash_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/exec_fixture_snapshot_hash.txt");

    if std::env::var("EL_UPDATE_GOLDEN").as_deref() == Ok("1") {
        fs::write(hash_path, format!("{}\n", h))?;
        return Ok(());
    }

    let cur = fs::read_to_string(hash_path)?;
    let want: u64 = cur.trim().parse().expect("hash u64");
    assert_eq!(
        want, h,
        "snapshot hash mismatch (regen with EL_UPDATE_GOLDEN=1)"
    );
    Ok(())
}
