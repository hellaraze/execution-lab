use anyhow::{Context, Result};
use blake3::Hasher;
use el_core::event::{Event, EventPayload, EventType};
use eventlog::EventLogReader;
use orderbook::OrderBook;

fn hash_book(book: &OrderBook) -> String {
    let mut h = Hasher::new();

    // BTreeMap iteration order is deterministic
    for (p, q) in book.bids.iter() {
        h.update(&p.0.to_le_bytes());
        h.update(&q.to_le_bytes());
    }
    h.update(b"|");

    for (p, q) in book.asks.iter() {
        h.update(&p.0.to_le_bytes());
        h.update(&q.to_le_bytes());
    }

    h.finalize().to_hex().to_string()
}

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "events_book.log".to_string());

    let mut r = EventLogReader::open(&path).with_context(|| format!("open log: {}", path))?;

    let mut book = OrderBook::new();
    let mut last_seq: Option<u64> = None;
    let mut n: u64 = 0;

    while let Some((env, payload_bytes)) = r.next()? {
        n += 1;
        last_seq = Some(env.seq);

        let ev: Event = serde_json::from_slice(&payload_bytes)
            .with_context(|| format!("parse core::Event json (seq={})", env.seq))?;

        match (&ev.event_type, &ev.payload) {
            (EventType::BookSnapshot, EventPayload::BookSnapshot { bids, asks }) => {
                book = OrderBook::new();
                book.apply_levels(bids, asks);
            }
            (EventType::BookDelta, EventPayload::BookDelta { bids, asks }) => {
                book.apply_levels(bids, asks);
            }
            (EventType::TickerBbo, EventPayload::TickerBbo { bid, ask }) => {
                // materialize 1-level book from BBO stream
                book = OrderBook::new();
                book.apply_levels(&vec![(*bid, 1.0)], &vec![(*ask, 1.0)]);
            }
            _ => {}
        }

        if n % 2000 == 0 {
            let bid = book.top_bid();
            let ask = book.top_ask();
            let h = hash_book(&book);
            println!("n={} seq={:?} bid={:?} ask={:?} hash={}", n, last_seq, bid, ask, h);
        }
    }

    let bid = book.top_bid();
    let ask = book.top_ask();
    let h = hash_book(&book);
    println!("FINAL n={} seq={:?} bid={:?} ask={:?} hash={}", n, last_seq, bid, ask, h);

    Ok(())
}
