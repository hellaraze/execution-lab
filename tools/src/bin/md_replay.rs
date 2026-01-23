use anyhow::{anyhow, Result};
use el_core::event::{Event, EventPayload, EventType};
use eventlog::EventLogReader;
use orderbook::OrderBook;
use std::collections::VecDeque;

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
struct StatsI64 {
    n: u64,
    min: i64,
    max: i64,
    sum: i128,
}
impl StatsI64 {
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
            Some((self.sum / self.n as i128) as i64)
        }
    }
}

#[derive(Default)]
struct StatsF64 {
    n: u64,
    min: f64,
    max: f64,
    sum: f64,
}
impl StatsF64 {
    fn push(&mut self, v: f64) {
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
        self.sum += v;
    }
    fn avg(&self) -> Option<f64> {
        if self.n == 0 {
            None
        } else {
            Some(self.sum / (self.n as f64))
        }
    }
}

fn percentile(sorted: &[f64], p: f64) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    let p = p.clamp(0.0, 1.0);
    let idx = ((sorted.len() - 1) as f64 * p).round() as usize;
    sorted.get(idx).copied()
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);

    let path = args
        .next()
        .unwrap_or_else(|| "/tmp/binance_depth.ndjson".to_string());

    let mut mode = Mode::Depth;

    let mut eps: f64 = 1e-9;
    let mut tick: f64 = 0.01;

    // Compare-mode knobs
    let mut window_ms: i64 = 250; // simple gate: skip if last book update too far (proc-time)
    let mut ring: usize = 4096; // ring size for top-of-book samples
    let mut max_print: u64 = 200; // max number of drift lines to print
    let mut print_threshold_ticks: f64 = 1.0; // print when drift_ticks >= threshold
    let mut print_threshold_usd: Option<f64> = None; // overrides ticks threshold via usd/tick

    // Percentiles memory cap (keep last N drift samples)
    let mut drift_keep: usize = 200_000;

    while let Some(a) = args.next() {
        match a.as_str() {
            "--mode" => {
                if let Some(v) = args.next() {
                    mode = parse_mode(&v);
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
            "--window-ms" => {
                if let Some(v) = args.next() {
                    window_ms = v.parse().unwrap_or(window_ms);
                }
            }
            "--ring" => {
                if let Some(v) = args.next() {
                    ring = v.parse().unwrap_or(ring);
                }
            }
            "--max-print" => {
                if let Some(v) = args.next() {
                    max_print = v.parse().unwrap_or(max_print);
                }
            }
            "--print-threshold-ticks" => {
                if let Some(v) = args.next() {
                    print_threshold_ticks = v.parse().unwrap_or(print_threshold_ticks);
                }
            }
            "--print-threshold-usd" => {
                if let Some(v) = args.next() {
                    print_threshold_usd = Some(v.parse().unwrap_or(0.0));
                }
            }
            "--drift-keep" => {
                if let Some(v) = args.next() {
                    drift_keep = v.parse().unwrap_or(drift_keep);
                }
            }
            // Back-compat: old flag name kept, now aliases to --max-print
            "--max-mismatch" => {
                if let Some(v) = args.next() {
                    max_print = v.parse().unwrap_or(max_print);
                }
            }
            _ => {}
        }
    }

    if tick <= 0.0 {
        return Err(anyhow!("--tick must be > 0"));
    }
    let effective_print_threshold_ticks = match print_threshold_usd {
        Some(u) => (u / tick).max(0.0),
        None => print_threshold_ticks,
    };

    let mut r = EventLogReader::open(&path)?;
    let mut book = OrderBook::new();

    // counts
    let mut n_snap = 0u64;
    let mut n_delta = 0u64;
    let mut n_bbo = 0u64;

    let mut last_bbo: Option<(f64, f64)> = None;

    // perf
    let mut first_proc: Option<i64> = None;
    let mut last_proc: Option<i64> = None;

    // raw proc-ex (also acts as clock offset)
    let mut lat_raw = StatsI64::default();

    // compare stats
    let mut compared = 0u64;
    let mut skipped = 0u64;

    // nearest book tracking (proc-time)
    let mut tob_ring: VecDeque<(i64, f64, f64)> = VecDeque::new(); // (ts_proc, best_bid, best_ask)
    let mut nearest_dt_ns = StatsI64::default();

    // drift metrics (absolute diffs)
    let mut drift_bid = StatsF64::default();
    let mut drift_ask = StatsF64::default();
    let mut drift_mid = StatsF64::default();
    let mut drift_spread = StatsF64::default();

    // store drift in ticks for percentiles
    let mut drift_ticks_vec: Vec<f64> = Vec::new();

    // printing
    let mut printed = 0u64;

    // last book proc (for simple gating)
    let mut last_book_proc_ns: Option<i64> = None;

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
            lat_raw.push(d);
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

                last_book_proc_ns = Some(e.ts_proc.nanos);

                if let (Some((bb, _)), Some((ba, _))) = (book.top_bid(), book.top_ask()) {
                    tob_ring.push_back((e.ts_proc.nanos, bb, ba));
                    while tob_ring.len() > ring {
                        tob_ring.pop_front();
                    }
                }
            }
            (EventType::BookDelta, EventPayload::BookDelta { bids, asks }) => {
                if mode == Mode::Bbo {
                    continue;
                }
                book.apply_levels(bids, asks);
                book.check_invariants().map_err(|x| anyhow!(x))?;
                n_delta += 1;

                last_book_proc_ns = Some(e.ts_proc.nanos);

                if let (Some((bb, _)), Some((ba, _))) = (book.top_bid(), book.top_ask()) {
                    tob_ring.push_back((e.ts_proc.nanos, bb, ba));
                    while tob_ring.len() > ring {
                        tob_ring.pop_front();
                    }
                }
            }
            (EventType::TickerBbo, EventPayload::TickerBbo { bid, ask }) => {
                if mode == Mode::Depth {
                    continue;
                }
                n_bbo += 1;
                last_bbo = Some((*bid, *ask));

                if mode == Mode::Compare {
                    if tob_ring.is_empty() {
                        skipped += 1;
                        continue;
                    }

                    if let Some(lb) = last_book_proc_ns {
                        let dt_ns = (e.ts_proc.nanos - lb).abs();
                        if dt_ns > window_ms * 1_000_000 {
                            skipped += 1;
                            continue;
                        }
                    }

                    // nearest sample by proc time
                    let t = e.ts_proc.nanos;
                    let mut best: Option<(i64, f64, f64)> = None;
                    let mut best_dt: i64 = i64::MAX;

                    for (tsb, bb, ba) in tob_ring.iter() {
                        let dt = (*tsb - t).abs();
                        if dt < best_dt {
                            best_dt = dt;
                            best = Some((*tsb, *bb, *ba));
                        }
                    }

                    let Some((_tsb, bb, ba)) = best else {
                        skipped += 1;
                        continue;
                    };

                    compared += 1;
                    nearest_dt_ns.push(best_dt);

                    let d_bid = (bb - *bid).abs();
                    let d_ask = (ba - *ask).abs();

                    let mid_book = (bb + ba) * 0.5;
                    let mid_tick = (*bid + *ask) * 0.5;
                    let d_mid = (mid_book - mid_tick).abs();

                    let spread_book = (ba - bb).abs();
                    let spread_tick = (*ask - *bid).abs();
                    let d_spread = (spread_book - spread_tick).abs();

                    drift_bid.push(d_bid);
                    drift_ask.push(d_ask);
                    drift_mid.push(d_mid);
                    drift_spread.push(d_spread);

                    let drift_ticks = (d_bid.max(d_ask) / tick).max(0.0);

                    if drift_ticks_vec.len() >= drift_keep {
                        let drop_n = drift_keep / 10 + 1;
                        drift_ticks_vec.drain(0..drop_n.min(drift_ticks_vec.len()));
                    }
                    drift_ticks_vec.push(drift_ticks);

                    if printed < max_print && drift_ticks + eps >= effective_print_threshold_ticks {
                        printed += 1;
                        println!(
                            "BBO_DRIFT sym={} tick={} drift_ticks={:.3} d_bid={:.8} d_ask={:.8} d_mid={:.8} d_spread={:.8} book_bid={} book_ask={} event_bid={} event_ask={} nearest_dt_ms={:.3}",
                            e.symbol,
                            tick,
                            drift_ticks,
                            d_bid,
                            d_ask,
                            d_mid,
                            d_spread,
                            bb,
                            ba,
                            bid,
                            ask,
                            (best_dt as f64) / 1e6
                        );
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

    let total_events = n_snap + n_delta + n_bbo;
    let eps_per_sec = duration_ns
        .map(|ns| (total_events as f64) / ((ns as f64) / 1e9))
        .filter(|v| v.is_finite());

    let lat_tuple = if lat_raw.n == 0 {
        None
    } else {
        Some((lat_raw.min, lat_raw.avg(), lat_raw.max))
    };
    let near_tuple = if nearest_dt_ns.n == 0 {
        None
    } else {
        Some((nearest_dt_ns.min, nearest_dt_ns.avg(), nearest_dt_ns.max))
    };

    let mut drift_sorted = drift_ticks_vec.clone();
    drift_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let p50 = percentile(&drift_sorted, 0.50);
    let p90 = percentile(&drift_sorted, 0.90);
    let p99 = percentile(&drift_sorted, 0.99);
    let p999 = percentile(&drift_sorted, 0.999);

    let drift_bid_tuple = if drift_bid.n == 0 {
        None
    } else {
        Some((drift_bid.min, drift_bid.avg(), drift_bid.max))
    };
    let drift_ask_tuple = if drift_ask.n == 0 {
        None
    } else {
        Some((drift_ask.min, drift_ask.avg(), drift_ask.max))
    };
    let drift_mid_tuple = if drift_mid.n == 0 {
        None
    } else {
        Some((drift_mid.min, drift_mid.avg(), drift_mid.max))
    };
    let drift_spread_tuple = if drift_spread.n == 0 {
        None
    } else {
        Some((drift_spread.min, drift_spread.avg(), drift_spread.max))
    };

    println!(
        "OK md_replay mode={mode:?} tick={tick} ring={ring} window_ms={window_ms} snapshots={n_snap} deltas={n_delta} bbo={n_bbo} compared={compared} skipped={skipped} printed={printed}/{max_print} last_bbo={last_bbo:?} hash64={hash64} eps={eps:?} latency_raw_ns={lat:?} nearest_dt_ns={near:?} drift_abs_bid={db:?} drift_abs_ask={da:?} drift_abs_mid={dm:?} drift_abs_spread={ds:?} drift_ticks_p50={p50:?} drift_ticks_p90={p90:?} drift_ticks_p99={p99:?} drift_ticks_p999={p999:?}",
        mode = mode,
        tick = tick,
        ring = ring,
        window_ms = window_ms,
        n_snap = n_snap,
        n_delta = n_delta,
        n_bbo = n_bbo,
        compared = compared,
        skipped = skipped,
        printed = printed,
        max_print = max_print,
        last_bbo = last_bbo,
        hash64 = book.state_hash64(),
        eps = eps_per_sec,
        lat = lat_tuple,
        near = near_tuple,
        db = drift_bid_tuple,
        da = drift_ask_tuple,
        dm = drift_mid_tuple,
        ds = drift_spread_tuple,
        p50 = p50,
        p90 = p90,
        p99 = p99,
        p999 = p999,
    );

    Ok(())
}
