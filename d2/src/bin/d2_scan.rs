use clap::Parser;

#[cfg(feature = "replay-ro")]
use d2::{compute_signal, DecisionReason, Fees, Thresholds};

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
    let mut obs = d2::obs::Obs::open(a.obs_out.as_deref());

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

    let _counts = std::collections::BTreeMap::<DecisionReason, u64>::new();

    while let Some((_env, payload)) = r.read_next()? {
        let e: Event = serde_json::from_slice(&payload)?;
        let ts = d2::obs::ts_from_event(&e);

        let (bid, ask) = match &e.payload {
            EventPayload::TickerBbo { bid, ask } => (*bid, *ask),
            _ => continue,
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
            e.instrument.clone(),
            ts,
            s.decision,
            s.net_edge_bps,
            s.reason,
        );
    }

    println!("d2_scan done top_n={}", a.top_n);
    Ok(())
}
