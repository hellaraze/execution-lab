use el_contracts::v1::{ExCommand, RiskEngine as RiskEngineContract, Side};
use el_core::event::Exchange;
use risk::{RiskConfig, RiskEngine};

#[test]
fn risk_engine_implements_contract() {
    let cfg = RiskConfig::default();
    let mut eng = RiskEngine::new(cfg);
    let _name = RiskEngineContract::name(&eng);

    // smoke: check accepts a basic command by default
    let cmd = ExCommand::Place {
        instrument: el_core::instrument::InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
        client_order_id: 1,
        side: Side::Bid,
        px: 100.0,
        qty: 1.0,
    };
    let r = RiskEngineContract::check(&mut eng, &cmd);
    assert!(r.is_ok());
}

#[test]
fn max_order_notional_guard() {
    let cfg = RiskConfig {
        max_order_notional: Some(50.0),
    };
    let mut eng = RiskEngine::new(cfg);

    let cmd = ExCommand::Place {
        instrument: el_core::instrument::InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
        client_order_id: 1,
        side: Side::Bid,
        px: 100.0,
        qty: 1.0,
    };

    let r = RiskEngineContract::check(&mut eng, &cmd);
    assert!(r.is_err());
}
