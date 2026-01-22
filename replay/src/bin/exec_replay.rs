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
    let mut n_total: u64 = 0;

    while let Some((env, payload_bytes)) = r.next()? {
        n_total += 1;
        let ev: Event = serde_json::from_slice(&payload_bytes)
            .with_context(|| format!("parse core::Event json (seq={})", env.seq))?;
        if let Some(x) = adapt_exec(&ev) {
            exec_events.push(x);
        }
    }

    if exec_events.is_empty() {
        println!("EXEC n_total={} n_exec_events=0", n_total);
        return Ok(());
    }

    let (stores, hash) = build_snapshot_multi(&exec_events).context("build exec snapshot multi")?;
    println!(
        "EXEC n_total={} n_exec_events={} n_instruments={} hash={}",
        n_total,
        exec_events.len(),
        stores.len(),
        hash
    );

    for (ik, store) in &stores {
        // cheap summary: count ids by scanning events for this instrument
        let mut ids: Vec<u64> = exec_events
            .iter()
            .filter(|e| e.instrument() == ik)
            .filter_map(|ev| match ev {
                ExecEv::OrderCreated { id, .. }
                | ExecEv::OrderValidated { id, .. }
                | ExecEv::OrderSent { id, .. }
                | ExecEv::OrderAcked { id, .. }
                | ExecEv::OrderFill { id, .. }
                | ExecEv::OrderCancelRequested { id, .. }
                | ExecEv::OrderCancelled { id, .. }
                | ExecEv::OrderRejected { id, .. }
                | ExecEv::OrderExpired { id, .. } => Some(id.0),
            })
            .collect();
        ids.sort_unstable();
        ids.dedup();

        let mut n_ack = 0u64;
        let mut n_fill = 0u64;
        let mut n_cancel = 0u64;
        let mut n_reject = 0u64;

        for id_u in &ids {
            let id = exec::events::OrderId(*id_u);
            if let Some(v) = store.view(id) {
                let s = format!("{:?}", v.state);
                if s == "Acknowledged" { n_ack += 1; }
                if s == "PartiallyFilled" { n_fill += 1; }
                if s == "Cancelled" { n_cancel += 1; }
                if s == "Rejected" { n_reject += 1; }
            }
        }

        println!(
            "INSTR {}:{} orders={} ack={} pf={} cancelled={} rejected={}",
            ik.exchange, ik.symbol, ids.len(), n_ack, n_fill, n_cancel, n_reject
        );
    }

    Ok(())
}
