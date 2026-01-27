use el_core::event::Exchange;
use el_core::instrument::InstrumentKey;
use strategy::registry::build_default_registry;
use strategy_sdk::{StrategyContext, StrategyInput};

fn main() {
    let reg = build_default_registry();

    println!("strategies:");
    for name in reg.list() {
        println!(" - {}", name);
    }

    let s = reg.get("always_no_gas").unwrap();

    let ctx = StrategyContext {
        instrument: InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
    };

    // minimal valid input (ignored by AlwaysNoGas)
    let input = StrategyInput {
        buy_price: 100.10,
        sell_price: 100.00,
        buy_is_maker: false,
        sell_is_maker: false,

        buy_maker_bps: 0.0,
        buy_taker_bps: 0.0007,
        buy_rebate_bps: 0.0,

        sell_maker_bps: 0.0,
        sell_taker_bps: 0.0007,
        sell_rebate_bps: 0.0,

        epsilon: 0.0,
        min_edge_bps: 2.0,
    };

    let (d, reason) = s.compute(&ctx, &input);
    println!("compute: decision={:?} reason={:?}", d, reason);
}
