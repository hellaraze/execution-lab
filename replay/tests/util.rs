use anyhow::{Context, Result};
use blake3::Hasher;
use el_core::event::{Event, EventPayload, EventType};
use eventlog::EventLogReader;
use orderbook::OrderBook;

fn chain_step(prev: u64, state: u64) -> u64 {
    let mut h = Hasher::new();
    h.update(b"chain:v1|");
    h.update(&prev.to_le_bytes());
    h.update(&state.to_le_bytes());
    let out = h.finalize();
    u64::from_le_bytes(out.as_bytes()[0..8].try_into().unwrap())
}

pub fn run_and_collect_chain_hashes(path: &str, max_events: usize) -> Result<Vec<u64>> {
    let mut r = EventLogReader::open(path).with_context(|| format!("open log: {}", path))?;
    let mut book = OrderBook::new();

    let mut out: Vec<u64> = Vec::new();
    let mut prev_chain: u64 = 0;
    let mut n: usize = 0;
    let mut last_env_seq: Option<u64> = None;

    while let Some((env, payload_bytes)) = r.read_next()? {
        if let Some(prev) = last_env_seq {
            if env.seq != prev + 1 {
                anyhow::bail!(
                    "GAP DETECTED: missing envelope seqs {}..{}",
                    prev + 1,
                    env.seq - 1
                );
            }
        }
        last_env_seq = Some(env.seq);

        let ev: Event = serde_json::from_slice(&payload_bytes).with_context(|| {
            format!(
                "parse core::Event json (step={} env.seq={})",
                n + 1,
                env.seq
            )
        })?;

        match (&ev.event_type, &ev.payload) {
            (EventType::BookSnapshot, EventPayload::BookSnapshot { bids, asks }) => {
                book = OrderBook::new();
                book.apply_levels(bids, asks);
            }
            (EventType::BookDelta, EventPayload::BookDelta { bids, asks }) => {
                book.apply_levels(bids, asks);
            }
            _ => {}
        }

        if let Err(e) = book.check_invariants() {
            let bid = book.top_bid();
            let ask = book.top_ask();
            let state = book.state_hash64();
            let ch = chain_step(prev_chain, state);
            anyhow::bail!(
                "INVARIANT FAIL step={} env.seq={} event_type={:?} bid={:?} ask={:?} prev_chain={} state_hash64={} chain_hash={} err={}",
                n + 1, env.seq, ev.event_type, bid, ask, prev_chain, state, ch, e
            );
        }

        let state = book.state_hash64();
        let ch = chain_step(prev_chain, state);
        out.push(ch);
        prev_chain = ch;

        n += 1;
        if n >= max_events {
            break;
        }
    }

    Ok(out)
}
