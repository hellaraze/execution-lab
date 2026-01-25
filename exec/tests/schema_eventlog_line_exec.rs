use std::fs;

use anyhow::Result;

use eventlog::reader::EventLogReader;
use eventlog::writer::{Durability, EventLogWriter};

use exec::events::ExecEvent;
use exec::events::OrderId;
use exec::util::instrument::InstrumentKey;

fn now_ns() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

#[test]
fn schema_eventlog_line_exec_golden() -> Result<()> {
    let path = format!(
        "{}/tests/data/_tmp_schema_eventlog_line_exec.eventlog",
        env!("CARGO_MANIFEST_DIR")
    );
    let _ = std::fs::remove_file(&path);

    // FIX: correct ctor + durability name; set stream explicitly
    let mut w = EventLogWriter::open_append(&path, "el:exec", Durability::Buffered)?;

    let ev = ExecEvent::OrderCreated {
        instrument: InstrumentKey::new("binance", "BTCUSDT"),
        id: OrderId(1),
    };

    let payload = serde_json::to_vec(&ev)?;
    w.append_bytes("event", now_ns(), &payload)?;
    w.flush()?;

    let mut r = EventLogReader::open(&path)?;
    let (env, payload2) = r.read_next()?.expect("one line");

    // normalize dynamic fields
    let mut env_v = serde_json::to_value(&env)?;
    if let Some(m) = env_v.as_object_mut() {
        m.insert("ts_ns".to_string(), serde_json::json!(0u64));
        // stream/kind/seq/payload_b64/checksum stay frozen
    }

    let payload_v: serde_json::Value = serde_json::from_slice(&payload2)?;

    let out = serde_json::json!({
        "envelope": env_v,
        "payload": payload_v,
    });

    let s = serde_json::to_string_pretty(&out)?;

    let golden_path = format!(
        "{}/tests/data/schema_eventlog_line_exec.json",
        env!("CARGO_MANIFEST_DIR")
    );

    if std::env::var("EL_UPDATE_GOLDEN").as_deref() == Ok("1") {
        let dir = std::path::Path::new(&golden_path).parent().unwrap();
        fs::create_dir_all(dir).expect("create golden dir");
        fs::write(&golden_path, &s).expect("write golden");
        return Ok(());
    }

    let cur =
        fs::read_to_string(&golden_path).expect("read golden (set EL_UPDATE_GOLDEN=1 to regen)");
    assert_eq!(
        cur, s,
        "schema mismatch: run `EL_UPDATE_GOLDEN=1 cargo test -q -p replay --test schema_eventlog_line_exec schema_eventlog_line_exec_golden` and commit updated golden ONLY if intentional"
    );

    Ok(())
}
