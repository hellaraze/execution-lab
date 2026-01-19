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

fn main() -> Result<()> {
    let log_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "replay/tests/data/golden_events_book.log".to_string());
    let max_events: usize = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(200);

    let mut r = EventLogReader::open(&log_path).with_context(|| format!("open log: {}", log_path))?;
    let mut book = OrderBook::new();

    let mut prev_chain: u64 = 0;
    let mut n: usize = 0;

    println!("# chain hashes generated from: {}", log_path);

    while let Some((env, payload_bytes)) = r.next()? {
        let ev: Event = serde_json::from_slice(&payload_bytes)
            .with_context(|| format!("parse core::Event json (seq={})", env.seq))?;

        match (&ev.event_type, &ev.payload) {
            (EventType::BookSnapshot, EventPayload::BookSnapshot { bids, asks }) => {
                book = OrderBook::new();
                book.apply_levels(&bids, &asks);
            }
            (EventType::BookDelta, EventPayload::BookDelta { bids, asks }) => {
                book.apply_levels(&bids, &asks);
            }
            _ => {}
        }

        book.check_invariants()
            .map_err(|e| anyhow::anyhow!("invariant fail at step={}: {}", n + 1, e))?;

        let state = book.state_hash64();
        let ch = chain_step(prev_chain, state);
        println!("{}", ch);
        prev_chain = ch;

        n += 1;
        if n >= max_events {
            break;
        }
    }

    Ok(())
}
