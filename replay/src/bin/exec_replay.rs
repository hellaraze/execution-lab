use anyhow::{Context, Result};
use el_core::event::{Event, EventType};
use eventlog::EventLogReader;

use exec::events::ExecEvent as ExecEv;
use exec::guard::ExecGuard;
use exec::order::snapshot::build_snapshot_multi;
use exec::order::types::OrderState;
use exec_bridge::adapter::adapt as adapt_exec;

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "events_book.log".to_string());

    let mut r = EventLogReader::open(&path).with_context(|| format!("open log: {}", path))?;

    let mut exec_events: Vec<ExecEv> = Vec::new();
    let mut guard = ExecGuard::new();

    let mut n_total: u64 = 0;
    let mut n_skipped: u64 = 0;

    while let Some((env, payload_bytes)) = r.next()? {
        n_total += 1;

        let ev: Event = serde_json::from_slice(&payload_bytes)
            .with_context(|| format!("parse core::Event json (seq={})", env.seq))?;

        match ev.event_type {
            EventType::GapDetected | EventType::ResyncStarted => guard.on_need_snapshot(),
            EventType::ResyncFinished | EventType::BookSnapshot => guard.on_snapshot(),
            _ => {}
        }

        if !guard.allow_exec() {
            n_skipped += 1;
            continue;
        }

        if let Some(x) = adapt_exec(&ev) {
            exec_events.push(x);
        }
    }

    if exec_events.is_empty() {
        println!(
            "EXEC n_total={} n_exec_events=0 skipped={} (guard blocks)",
            n_total, n_skipped
        );
        return Ok(());
    }

    let (stores, hash) = build_snapshot_multi(&exec_events).context("build exec snapshot multi")?;
    println!(
        "EXEC n_total={} skipped={} n_exec_events={} n_instruments={} hash={}",
        n_total,
        n_skipped,
        exec_events.len(),
        stores.len(),
        hash
    );

    for (ik, store) in &stores {
        // seen counters by scanning stream for this instrument
        let mut seen_ack = 0u64;
        let mut seen_fill = 0u64;
        let mut seen_cancel_req = 0u64;
        let mut seen_cancelled = 0u64;
        let mut seen_rejected = 0u64;

        for ev in exec_events.iter().filter(|e| e.instrument() == ik) {
            match ev {
                ExecEv::OrderAcked { .. } => seen_ack += 1,
                ExecEv::OrderFill { .. } => seen_fill += 1,
                ExecEv::OrderCancelRequested { .. } => seen_cancel_req += 1,
                ExecEv::OrderCancelled { .. } => seen_cancelled += 1,
                ExecEv::OrderRejected { .. } => seen_rejected += 1,
                _ => {}
            }
        }

        // final state summary: per-order final states
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

        let mut final_ack = 0u64;
        let mut final_pf = 0u64;
        let mut final_filled = 0u64;
        let mut final_cancelled = 0u64;
        let mut final_rejected = 0u64;

        for id_u in &ids {
            let id = exec::events::OrderId(*id_u);
            if let Some(v) = store.view(id) {
                match v.state {
                    OrderState::Acknowledged => final_ack += 1,
                    OrderState::PartiallyFilled => final_pf += 1,
                    OrderState::Filled => final_filled += 1,
                    OrderState::Cancelled => final_cancelled += 1,
                    OrderState::Rejected => final_rejected += 1,
                    _ => {}
                }
            }
        }

        println!(
            "INSTR {}:{} orders={} final_ack={} final_pf={} final_filled={} final_cancelled={} final_rejected={} | seen_ack={} seen_fill={} seen_cancel_req={} seen_cancelled={} seen_rejected={}",
            ik.exchange,
            ik.symbol,
            ids.len(),
            final_ack,
            final_pf,
            final_filled,
            final_cancelled,
            final_rejected,
            seen_ack,
            seen_fill,
            seen_cancel_req,
            seen_cancelled,
            seen_rejected
        );
    }

    Ok(())
}
