use anyhow::Result;
use eventlog::reader::EventLogReader;

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/binance_depth.ndjson".to_string());

    let mut r = EventLogReader::open(&path)?;

    let mut n = 0usize;
    while let Some((env, payload)) = r.read_next()? {
        println!(
            "seq={} ts_ns={} kind={} stream={} payload_bytes={}",
            env.seq,
            env.ts_ns,
            env.kind,
            env.stream,
            payload.len()
        );
        n += 1;
        if n >= 5 {
            break;
        }
    }

    Ok(())
}
