use anyhow::{anyhow, Result};
use el_core::event::{Event, EventPayload, EventType};
use eventlog::EventLogReader;
use orderbook::OrderBook;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Depth,
    Bbo,
    Compare,
}

fn parse_mode(s: &str) -> Mode {
    match s {
        "depth" => Mode::Depth,
        "bbo" => Mode::Bbo,
        "compare" => Mode::Compare,
        _ => Mode::Depth,
    }
}

#[derive(Default)]
struct LatStats {
    n: u64,
    min: i64,
    max: i64,
    sum: i128,
}

impl LatStats {
    fn push(&mut self, v: i64) {
        if self.n == 0 {
            self.min = v;
            self.max = v;
        } else {
            if v < self.min {
                self.min = v;
            }
            if v > self.max {
                self.max = v;
            }
        }
        self.n += 1;
        self.sum += v as i128;
    }
    fn avg(&self) -> Option<i64> {
        if self.n == 0 {
            None
        } else {
            Some((self.sum / (self.n as i128)) as i64)
        }
    }
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);

    let path = args
        .next()
        .unwrap_or_else(|| "/tmp/binance_depth.ndjson".to_string());

    let mut mode = Mode::Depth;
    let mut max_mismatch: u64 = 200;
    let mut eps: f64 = 1e-9;
    let mut tick: f64 = 0.01;

    while let Some(a) = args.next() {
        match a.as_str() {
            "--mode" => {
                if let Some(v) = args.next() {
                    mode = parse_mode(&v);
                }
            }
            "--max-mismatch" => {
                if let Some(v) = args.next() {
                    max_mismatch = v.parse().unwrap_or(max_mismatch);
                }
            }
            "--eps" => {
                if let Some(v) = args.next() {
                    eps = v.parse().unwrap_or(eps);
                }
            }
            "--tick" => {
                if let Some(v) = args.next() {
                    tick = v.parse().unwrap_or(tick);
                }
            }
            _ => {}
        }
    }

    let mut r = EventLogReader::open(&path)?;

    let mut book = OrderBook::new();

    let mut mismatch = 0u64;

    let mut n_snap = 0u64;
    let mut n_delta = 0u64;
    let mut n_bbo = 0u64;

    let mut last_bbo: Option<(f64, f64)> = None;

    // perf/latency
    let mut first_proc: Option<i64> = None;
    let mut last_proc: Option<i64> = None;

    let mut lat = LatStats::default();

    loop {
        let Some((env, payload)) = r.next()? else {
            break;
        };
        if env.kind != "event" {
            continue;
        }
        let e: Event = serde_json::from_slice(&payload)?;

        first_proc.get_or_insert(e.ts_proc.nanos);
        last_proc = Some(e.ts_proc.nanos);

        if let Some(ts_ex) = e.ts_exchange {
            let d = e.ts_proc.nanos - ts_ex.nanos;
            lat.push(d);
        }

        match (&e.event_type, &e.payload) {
            (EventType::BookSnapshot, EventPayload::BookSnapshot { bids, asks }) => {
                if mode == Mode::Bbo {
                    continue;
                }
                book = OrderBook::new();
                book.apply_levels(bids, asks);
                book.check_invariants().map_err(|x| anyhow!(x))?;
                n_snap += 1;
            }
            (EventType::BookDelta, EventPayload::BookDelta { bids, asks }) => {
                if mode == Mode::Bbo {
                    continue;
                }
                book.apply_levels(bids, asks);
                book.check_invariants().map_err(|x| anyhow!(x))?;
                n_delta += 1;
            }
            (EventType::TickerBbo, EventPayload::TickerBbo { bid, ask }) => {
                if mode == Mode::Depth {
                    continue;
                }
                n_bbo += 1;
                last_bbo = Some((*bid, *ask));

                if mode == Mode::Compare {
                    let top_bid = book.top_bid().map(|(p, _q)| p);
                    let top_ask = book.top_ask().map(|(p, _q)| p);

                    if let (Some(tb), Some(ta)) = (top_bid, top_ask) {
                        let ok = (tb - *bid).abs() <= tick + eps && (ta - *ask).abs() <= tick + eps;
                        if !ok {
                            mismatch += 1;
                            println!(
                                "BBO_MISMATCH seq={:?} sym={} event_bid={} event_ask={} book_bid={} book_ask={}",
                                e.seq, e.symbol, bid, ask, tb, ta
                            );
                            if mismatch >= max_mismatch {
                                return Err(anyhow!(
                                    "too many mismatches (>= {max_mismatch}); abort"
                                ));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let duration_ns = match (first_proc, last_proc) {
        (Some(a), Some(b)) if b > a => Some((b - a) as u64),
        _ => None,
    };

    let eps_per_sec = duration_ns
        .map(|ns| ((n_snap + n_delta + n_bbo) as f64) / ((ns as f64) / 1e9))
        .filter(|v| v.is_finite());

    println!(
        "OK replay tick={} mode={:?} snapshots={} deltas={} bbo={} mismatches={} last_bbo={:?} hash64={} eps={:?} latency_ns(min/avg/max)={:?}/{:?}/{:?}",
        tick,
        mode,
        n_snap,
        n_delta,
        n_bbo,
        mismatch,
        last_bbo,
        book.state_hash64(),
        eps_per_sec,
        if lat.n==0 { None } else { Some(lat.min) },
        lat.avg(),
        if lat.n==0 { None } else { Some(lat.max) },
    );

    Ok(())
}
