use clap::Parser;

#[cfg(feature = "replay-ro")]
use d2::{compute_signal, Fees, Thresholds};

#[cfg(feature = "replay-ro")]
use el_core::event::{Event, EventPayload};

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
    let mut obs = d2::obs::Obs::open(a.obs_out.as_deref());

    let mut r = eventlog::EventLogReader::open(&a.path)?;
    let mut last: Option<Event> = None;
    while let Some((_env, payload)) = r.read_next()? {
        let e: Event = serde_json::from_slice(&payload)?;
        last = Some(e);
    }
    let last = last.expect("events");

    let ts = d2::obs::ts_from_event(&last);

    let (bid, ask) = match &last.payload {
        EventPayload::TickerBbo { bid, ask } => (*bid, *ask),
        _ => (0.0, 0.0),
    };

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

    let s = compute_signal(
        d2::SpreadInput {
            buy_price: bid,
            sell_price: ask,
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

    d2::obs::emit_decision_at(
        &mut obs,
        last.instrument.clone(),
        ts,
        s.decision,
        s.net_edge_bps,
        s.reason,
    );

    println!(
        "d2_gas decision={:?} reason={:?} edge_bps={:.4} net={:.8} raw={:.8}",
        s.decision, s.reason, s.net_edge_bps, s.net_spread, s.raw_spread
    );

    Ok(())
}
