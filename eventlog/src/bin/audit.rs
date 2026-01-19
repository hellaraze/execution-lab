use anyhow::Result;
use eventlog::reader::EventLogReader;

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/el_test.log".to_string());

    let mut r = EventLogReader::open(&path)?;
    let mut n = 0u64;
    let mut last_seq: Option<u64> = None;

    while let Some((env, _payload)) = r.next()? {
        if let Some(prev) = last_seq {
            if env.seq != prev + 1 {
                anyhow::bail!(
                    "seq gap: prev={} current={} (line={})",
                    prev,
                    env.seq,
                    n + 1
                );
            }
        }
        last_seq = Some(env.seq);
        n += 1;
    }

    eprintln!("OK: lines={} last_seq={:?}", n, last_seq);
    Ok(())
}
