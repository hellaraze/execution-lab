use anyhow::Result;
use eventlog::{EventLogReader, EventLogWriter};
use serde_json::json;

fn main() -> Result<()> {
    let path = "/tmp/el_events.ndjson";
    let _ = std::fs::remove_file(path);

    let mut w = EventLogWriter::open_append(path, "smoke:test")?;
    w.append_json_value("depth", 111, &json!({"bids":[[1.0,2.0]],"asks":[[3.0,4.0]]}))?;
    w.append_json_value("bbo", 222, &json!({"bid":1.0,"ask":3.0}))?;
    w.append_json_value("trade", 333, &json!({"px":2.0,"qty":0.5}))?;
    w.flush()?;

    let mut r = EventLogReader::open(path)?;
    while let Some(env) = r.next()? {
        println!("seq={} ts={} kind={} stream={} payload_len={}",
            env.seq, env.ts_ns, env.kind, env.stream, env.payload.len()
        );
    }
    Ok(())
}
