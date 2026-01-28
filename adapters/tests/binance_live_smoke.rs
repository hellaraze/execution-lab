use adapters::binance_live::{BinanceLiveAdapter, LiveMode};
use el_core::event::Exchange;
use el_core::instrument::InstrumentKey;
use el_risk::limits::RiskLimits;
use exec::adapter::{ExecAdapter, PlaceOrder, Side};
use exec::events::OrderId;

#[test]
fn risk_blocks_large_notional() {
    let limits = RiskLimits { max_notional: 10.0 };
    let mut a = BinanceLiveAdapter::new(LiveMode::DryRun, limits);

    let res = a.place_order(PlaceOrder {
        instrument: InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
        order_id: OrderId(1),
        price: 1000.0,
        qty: 1.0,
        side: Side::Buy,
    });

    assert!(matches!(res, exec::adapter::ExecResult::Rejected { .. }));
}

#[test]
fn kill_switch_blocks() {
    let limits = RiskLimits {
        max_notional: 1_000_000_000.0,
    };
    let mut a = BinanceLiveAdapter::new(LiveMode::DryRun, limits);
    let k = a.kill_switch();
    k.kill();

    let res = a.place_order(PlaceOrder {
        instrument: InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
        order_id: OrderId(2),
        price: 100.0,
        qty: 1.0,
        side: Side::Buy,
    });

    match res {
        exec::adapter::ExecResult::Rejected { reason } => assert!(reason.contains("KILL_SWITCH")),
        _ => panic!("expected rejected"),
    }
}
