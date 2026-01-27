use clap::Parser;

#[cfg(feature = "replay-ro")]
use d2::ro::extract_last_bbo;
#[cfg(feature = "replay-ro")]
use d2::{compute_signal, Fees, Thresholds};

#[cfg(feature = "replay-ro")]
use el_core::event::Event;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    obs_out: Option<String>,

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
    let mut obs = d2::obs::Obs::open(a.obs_out.as_deref());

    fn read_events(path: &str) -> anyhow::Result<Vec<Event>> {
        let mut r = eventlog::EventLogReader::open(path)?;
        let mut out = Vec::new();
        while let Some((_env, payload)) = r.read_next()? {
            let e: Event = serde_json::from_slice(&payload)?;
            out.push(e);
        }
        Ok(out)
    }

    let events_a = read_events(&a.a)?;
    let events_b = read_events(&a.b)?;

    let bbo_a = extract_last_bbo(&events_a).ok_or_else(|| anyhow::anyhow!("bbo a"))?;
    let mut bbo_b = extract_last_bbo(&events_b).ok_or_else(|| anyhow::anyhow!("bbo b"))?;

    // deterministic synthetic shift on B
    let shift = a.b_shift_bps / 10_000.0;
    bbo_b.bid *= 1.0 + shift;
    bbo_b.ask *= 1.0 + shift;

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

    // buy on A at ask, sell on B at bid
    let s = compute_signal(
        d2::SpreadInput {
            buy_price: bbo_a.ask,
            sell_price: bbo_b.bid,
            buy_is_maker: false,
            sell_is_maker: false,
        },
        buy_fees,
        sell_fees,
        Thresholds {
            epsilon: a.epsilon,
            min_edge_bps: a.min_edge_bps,
        },
    );

    let ts = el_core::time::Timestamp::new(0, el_core::time::TimeSource::Process);
    let instrument =
        el_core::instrument::InstrumentKey::new(el_core::event::Exchange::Binance, "PAIR");
    d2::obs::emit_decision_at(
        &mut obs,
        instrument,
        ts,
        s.decision,
        s.net_edge_bps,
        s.reason,
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
