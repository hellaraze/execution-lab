use clap::Parser;

#[cfg(feature = "replay-ro")]
use d2::ro::extract_last_bbo;
#[cfg(feature = "replay-ro")]
use d2::{compute_signal, Fees, Thresholds};

#[cfg(feature = "replay-ro")]
use el_core::event::Event;

#[derive(Parser, Debug)]
struct Args {
    /// eventlog A (buy on A)
    a: String,
    /// eventlog B (sell on B)
    b: String,

    #[arg(long, default_value_t = 0.0007)]
    buy_taker: f64,
    #[arg(long, default_value_t = 0.0007)]
    sell_taker: f64,
    #[arg(long, default_value_t = 0.0)]
    epsilon: f64,
    #[arg(long, default_value_t = 2.0)]
    min_edge_bps: f64,

    /// shift B prices by +bps AFTER extract_last_bbo (deterministic synthetic edge)
    #[arg(long, default_value_t = 0.0)]
    b_shift_bps: f64,
}

#[cfg(not(feature = "replay-ro"))]
fn main() -> anyhow::Result<()> {
    anyhow::bail!("run with --features replay-ro");
}

#[cfg(feature = "replay-ro")]
fn main() -> anyhow::Result<()> {
    let a = Args::parse();

    // read A
    let mut r_a = eventlog::EventLogReader::open(&a.a)?;
    let mut ev_a: Vec<Event> = Vec::new();
    while let Some((_env, payload)) = r_a.read_next()? {
        let e: Event = serde_json::from_slice(&payload)?;
        ev_a.push(e);
    }

    // read B
    let mut r_b = eventlog::EventLogReader::open(&a.b)?;
    let mut ev_b: Vec<Event> = Vec::new();
    while let Some((_env, payload)) = r_b.read_next()? {
        let e: Event = serde_json::from_slice(&payload)?;
        ev_b.push(e);
    }

    let bbo_a =
        extract_last_bbo(&ev_a).ok_or_else(|| anyhow::anyhow!("no BBO/book snapshot in A"))?;
    let mut bbo_b =
        extract_last_bbo(&ev_b).ok_or_else(|| anyhow::anyhow!("no BBO/book snapshot in B"))?;

    // deterministic synthetic shift on B (post-extract)
    if a.b_shift_bps != 0.0 {
        let k = 1.0 + (a.b_shift_bps / 10_000.0);
        bbo_b.bid *= k;
        bbo_b.ask *= k;
    }

    // BUY on A at ask, SELL on B at bid
    let s = compute_signal(
        d2::SpreadInput {
            buy_price: bbo_a.ask,
            sell_price: bbo_b.bid,
            buy_is_maker: false,
            sell_is_maker: false,
        },
        Fees {
            maker: 0.0,
            taker: a.buy_taker,
            rebate: 0.0,
        },
        Fees {
            maker: 0.0,
            taker: a.sell_taker,
            rebate: 0.0,
        },
        Thresholds {
            epsilon: a.epsilon,
            min_edge_bps: a.min_edge_bps,
        },
    );

    let tag = if matches!(s.decision, d2::GasDecision::Gas) {
        "GAS"
    } else {
        "NO_GAS"
    };

    println!(
        "{} reason={:?} edge_bps={:.4} net={:.8} raw={:.8} A(bid/ask)={:.2}/{:.2} B(bid/ask)={:.2}/{:.2} b_shift_bps={:.2}",
        tag,
        s.reason,
        s.net_edge_bps,
        s.net_spread,
        s.raw_spread,
        bbo_a.bid,
        bbo_a.ask,
        bbo_b.bid,
        bbo_b.ask,
        a.b_shift_bps
    );

    Ok(())
}
