use clap::Parser;
use d2::obs::{Obs, emit_decision_at, ts_from_event};
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

    path: String,
    #[arg(long, default_value_t = 0.0007)]
    buy_taker: f64,
    #[arg(long, default_value_t = 0.0007)]
    sell_taker: f64,
    #[arg(long, default_value_t = 0.0)]
    epsilon: f64,
    #[arg(long, default_value_t = 2.0)]
    min_edge_bps: f64,
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
    let mut events: Vec<Event> = Vec::new();
    while let Some((_env, payload)) = r.read_next()? {
        let e: Event = serde_json::from_slice(&payload)?;
        events.push(e);
    }

    let bbo =
        extract_last_bbo(&events).ok_or_else(|| anyhow::anyhow!("no BBO/book snapshot in log"))?;

    let s = compute_signal(
        d2::SpreadInput {
            buy_price: bbo.ask,
            sell_price: bbo.bid,
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
        "{} reason={:?} edge_bps={:.4} net={:.8} raw={:.8} bid={:.2} ask={:.2}",
        tag, s.reason, s.net_edge_bps, s.net_spread, s.raw_spread, bbo.bid, bbo.ask
    );

    emit_decision_at(&mut obs, ts_from_event(events.last().expect("events")), &format!("d2_gas decision={:?} reason={:?} edge_bps={:.4} net={:.8} raw={:.8}", s.decision, s.reason, s.net_edge_bps, s.net_spread, s.raw_spread));
    Ok(())
}
