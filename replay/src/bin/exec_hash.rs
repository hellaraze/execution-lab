use anyhow::{Context, Result};
use el_core::event::Event;
use eventlog::EventLogReader;

use exec_bridge::adapter::adapt as adapt_exec;
use exec::events::ExecEvent as ExecEv;
use exec::order::snapshot::build_snapshot_multi;

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "events_book.log".to_string());

    let mut r = EventLogReader::open(&path).with_context(|| format!("open log: {}", path))?;

    let mut exec_events: Vec<ExecEv> = Vec::new();
    let mut n: u64 = 0;

    while let Some((env, payload_bytes)) = r.next()? {
        n += 1;
        let ev: Event = serde_json::from_slice(&payload_bytes)
            .with_context(|| format!("parse core::Event json (seq={})", env.seq))?;

        if let Some(x) = adapt_exec(&ev) {
            exec_events.push(x);
        }
    }

    if exec_events.is_empty() {
        println!("EXEC n_events=0 (n_total={})", n);
        return Ok(());
    }

    let (stores, hash) = build_snapshot_multi(&exec_events).context("build exec snapshot multi")?;
    println!(
        "EXEC n_total={} n_events={} n_instruments={} hash={}",
        n,
        exec_events.len(),
        stores.len(),
        hash
    );

    Ok(())
}
