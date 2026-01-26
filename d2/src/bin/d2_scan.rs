use clap::Parser;
use d2::obs::{Obs, emit_decision_at, ts_from_event};
#[cfg(feature = "replay-ro")]
use d2::ro::extract_last_bbo;
#[cfg(feature = "replay-ro")]
use d2::{compute_signal, DecisionReason, Fees, Thresholds};

#[cfg(feature = "replay-ro")]
use el_core::event::Event;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    obs_out: Option<String>,

    path: String,
    #[arg(long, default_value_t = 0.0007)]
    buy_taker: f64,
    #[arg(long, default_value_t = 0.0007)]
    sell_taker: f64,
    #[arg(long, default_value_t = 0.0)]
    epsilon: f64,
    #[arg(long, default_value_t = 2.0)]
    min_edge_bps: f64,
    #[arg(long, default_value_t = 20)]
    top_n: usize,
}

#[cfg(not(feature = "replay-ro"))]
fn main() -> anyhow::Result<()> {
    anyhow::bail!("run with --features replay-ro");
}

#[cfg(feature = "replay-ro")]
fn main() -> anyhow::Result<()> {
    let a = Args::parse();

    let mut obs = Obs::open(a.obs_out.as_deref());
    let mut r = eventlog::EventLogReader::open(&a.path)?;
    let buy_fees = Fees {
        maker: 0.0,
        taker: a.buy_taker,
        rebate: 0.0,
    };
    let sell_fees = Fees {
        maker: 0.0,
        taker: a.sell_taker,
        rebate: 0.0,
    };
    let t = Thresholds {
        epsilon: a.epsilon,
        min_edge_bps: a.min_edge_bps,
    };

    let mut best: Vec<(f64, String)> = Vec::new();
    let mut counts = std::collections::BTreeMap::<DecisionReason, u64>::new();
    let mut window: Vec<Event> = Vec::new();

    while let Some((_env, payload)) = r.read_next()? {
        let e: Event = serde_json::from_slice(&payload)?;
        let e_ts = ts_from_event(&e);
        window.push(e);

        if let Some(bbo) = extract_last_bbo(&window) {
            let s = compute_signal(
                d2::SpreadInput {
                    buy_price: bbo.ask,
                    sell_price: bbo.bid,
                    buy_is_maker: false,
                    sell_is_maker: false,
                },
                buy_fees,
                sell_fees,
                t,
            );

            emit_decision_at(&mut obs, e_ts, &format!("d2_scan reason={:?} edge_bps={:.4} net={:.8} raw={:.8}", s.reason, s.net_edge_bps, s.net_spread, s.raw_spread));
            *counts.entry(s.reason).or_insert(0) += 1;

            best.push((
                s.net_edge_bps,
                format!(
                    "edge_bps={:.4} net={:.8} raw={:.8} reason={:?} bid={:.2} ask={:.2}",
                    s.net_edge_bps, s.net_spread, s.raw_spread, s.reason, bbo.bid, bbo.ask
                ),
            ));
        }

        if window.len() > 4096 {
            window.drain(0..2048);
        }
    }

    best.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    best.truncate(a.top_n);

    println!(
        "=== TOP {} by net_edge_bps (min_edge_bps={}) ===",
        a.top_n, a.min_edge_bps
    );
    for (i, (_edge, line)) in best.iter().enumerate() {
        println!("{:02}: {}", i + 1, line);
    }

    println!("\n=== REASON COUNTS ===");
    for (k, v) in counts.iter() {
        println!("{:?}: {}", k, v);
    }

    Ok(())
}
