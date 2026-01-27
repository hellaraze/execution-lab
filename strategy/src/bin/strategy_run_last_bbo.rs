use anyhow::Context;
use clap::Parser;

use d2::ro::extract_last_bbo;
use d2::{Fees, GasDecision, SpreadInput, Thresholds, compute_signal};

use el_core::event::Event;

use strategy::registry::build_default_registry;
use strategy_sdk::{StrategyContext, StrategyInput};

use el_core::event::Exchange;
use el_core::instrument::InstrumentKey;

#[derive(Parser, Debug)]
struct Args {
    path: String,

    // taker-only fee model by default (mirrors d2_gas)
    #[arg(long, default_value_t = 0.0007)]
    buy_taker: f64,
    #[arg(long, default_value_t = 0.0007)]
    sell_taker: f64,

    #[arg(long, default_value_t = 0.0)]
    epsilon: f64,
    #[arg(long, default_value_t = 2.0)]
    min_edge_bps: f64,

    // sanity guard: reject insane spreads (bps of buy_price)
    #[arg(long, default_value_t = 500.0)]
    max_spread_bps: f64,
}

fn die_invalid(msg: &str) -> ! {
    println!("INVALID_INPUT {}", msg);
    std::process::exit(2);
}

fn main() -> anyhow::Result<()> {
    let a = Args::parse();

    let mut r = eventlog::EventLogReader::open(&a.path)?;
    let mut events: Vec<Event> = Vec::new();
    while let Some((_env, payload)) = r.read_next()? {
        let e: Event = serde_json::from_slice(&payload)?;
        events.push(e);
    }

    let bbo = extract_last_bbo(&events).context("no BBO/book snapshot in log")?;

    // =========
    // F.5: input validation / invariants
    // =========
    if !bbo.bid.is_finite() || !bbo.ask.is_finite() {
        die_invalid("bid/ask not finite");
    }
    if bbo.bid <= 0.0 || bbo.ask <= 0.0 {
        die_invalid("bid/ask <= 0");
    }
    if bbo.bid > bbo.ask {
        die_invalid("bid > ask");
    }

    let buy_price = bbo.ask;
    let sell_price = bbo.bid;
    let raw_spread = sell_price - buy_price; // <= 0 normally on single book
    let spread_bps = (raw_spread / buy_price) * 10_000.0;
    if spread_bps.abs() > a.max_spread_bps {
        die_invalid("spread exceeds max_spread_bps");
    }

    let ctx = StrategyContext {
        instrument: InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
    };

    let input = StrategyInput {
        buy_price,
        sell_price,
        buy_is_maker: false,
        sell_is_maker: false,

        buy_maker_bps: 0.0,
        buy_taker_bps: a.buy_taker,
        buy_rebate_bps: 0.0,

        sell_maker_bps: 0.0,
        sell_taker_bps: a.sell_taker,
        sell_rebate_bps: 0.0,

        epsilon: a.epsilon,
        min_edge_bps: a.min_edge_bps,
    };

    // Run strategy
    let reg = build_default_registry();
    let strat = reg.get("d2_signal").context("strategy d2_signal missing")?;
    let (d_s, reason_s) = strat.compute(&ctx, &input);

    // Compute ground-truth metrics via d2 directly (invariant: must match strategy output)
    let si = SpreadInput {
        buy_price: input.buy_price,
        sell_price: input.sell_price,
        buy_is_maker: input.buy_is_maker,
        sell_is_maker: input.sell_is_maker,
    };
    let buy_fees = Fees {
        maker: input.buy_maker_bps,
        taker: input.buy_taker_bps,
        rebate: input.buy_rebate_bps,
    };
    let sell_fees = Fees {
        maker: input.sell_maker_bps,
        taker: input.sell_taker_bps,
        rebate: input.sell_rebate_bps,
    };
    let t = Thresholds {
        epsilon: input.epsilon,
        min_edge_bps: input.min_edge_bps,
    };

    let sig = compute_signal(si, buy_fees, sell_fees, t);

    // invariant: strategy == d2
    if format!("{:?}", d_s) != format!("{:?}", sig.decision)
        || format!("{:?}", reason_s) != format!("{:?}", sig.reason)
    {
        die_invalid("strategy output != d2 compute_signal (contract drift)");
    }

    let tag = if matches!(sig.decision, GasDecision::Gas) {
        "GAS"
    } else {
        "NO_GAS"
    };

    println!(
        "{} reason={:?} edge_bps={:.4} net={:.8} raw={:.8} bid={:.2} ask={:.2}",
        tag, sig.reason, sig.net_edge_bps, sig.net_spread, sig.raw_spread, bbo.bid, bbo.ask
    );

    Ok(())
}
