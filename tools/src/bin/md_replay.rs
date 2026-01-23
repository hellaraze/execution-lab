use anyhow::{anyhow, Result};
use el_core::event::{Event, EventPayload, EventType};
use eventlog::EventLogReader;
use orderbook::OrderBook;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let path = args
        .next()
        .unwrap_or_else(|| "/tmp/binance_depth.ndjson".to_string());

    let mut r = EventLogReader::open(&path)?;

    let mut book = OrderBook::new();

    let mut last_bbo: Option<(f64, f64)> = None;
    let mut mismatch = 0u64;

    let mut n_snap = 0u64;
    let mut n_delta = 0u64;
    let mut n_bbo = 0u64;

    loop {
        let Some((env, payload)) = r.next()? else {
            break;
        };
        if env.kind != "event" {
            continue;
        }
        let e: Event = serde_json::from_slice(&payload)?;

        match (&e.event_type, &e.payload) {
            (EventType::BookSnapshot, EventPayload::BookSnapshot { bids, asks }) => {
                book = OrderBook::new();
                book.apply_levels(bids, asks);
                book.check_invariants().map_err(|x| anyhow!(x))?;
                n_snap += 1;
            }
            (EventType::BookDelta, EventPayload::BookDelta { bids, asks }) => {
                book.apply_levels(bids, asks);
                book.check_invariants().map_err(|x| anyhow!(x))?;
                n_delta += 1;
            }
            (EventType::TickerBbo, EventPayload::TickerBbo { bid, ask }) => {
                n_bbo += 1;
                last_bbo = Some((*bid, *ask));

                // Compare with reconstructed top of book when available
                let top_bid = book.top_bid().map(|(p, _q)| p);
                let top_ask = book.top_ask().map(|(p, _q)| p);

                if let (Some(tb), Some(ta)) = (top_bid, top_ask) {
                    // NOTE: feed can be async; allow equal within epsilon
                    let eps = 1e-9;
                    let ok = (tb - *bid).abs() < eps && (ta - *ask).abs() < eps;
                    if !ok {
                        mismatch += 1;
                        println!(
                            "BBO_MISMATCH seq={:?} sym={} event_bid={} event_ask={} book_bid={} book_ask={}",
                            e.seq, e.symbol, bid, ask, tb, ta
                        );
                        if mismatch >= 20 {
                            return Err(anyhow!("too many mismatches (>=20); abort"));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    println!(
        "OK replay: snapshots={} deltas={} bbo={} mismatches={} last_bbo={:?} hash64={}",
        n_snap,
        n_delta,
        n_bbo,
        mismatch,
        last_bbo,
        book.state_hash64()
    );

    Ok(())
}
