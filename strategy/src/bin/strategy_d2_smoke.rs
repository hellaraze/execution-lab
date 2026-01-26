use strategy::registry::build_default_registry;
use strategy_sdk::{StrategyContext, StrategyInput};
use el_core::instrument::InstrumentKey;
use el_core::event::Exchange;

fn main() {
    let ctx = StrategyContext {
        instrument: InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
    };

    // demo: buy at 100.10, sell at 100.20
    let input = StrategyInput {
        buy_price: 100.10,
        sell_price: 100.20,
        buy_is_maker: false,
        sell_is_maker: false,

        buy_maker_bps: 1.0,
        buy_taker_bps: 2.0,
        buy_rebate_bps: 0.0,

        sell_maker_bps: 1.0,
        sell_taker_bps: 2.0,
        sell_rebate_bps: 0.0,

        epsilon: 0.0,
        min_edge_bps: 0.0,
    };

    let reg = build_default_registry();
    let s = reg.get("d2_signal").unwrap();
    let (d, reason) = s.compute(&ctx, &input);

    println!("d2_signal: decision={:?} reason={:?}", d, reason);
}
