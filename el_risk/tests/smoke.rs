use el_core::event::Exchange;
use el_core::instrument::InstrumentKey;
use el_core::time::{TimeSource, Timestamp};
use el_risk::contract::*;
use el_risk::decision::SimpleRisk;
use el_risk::limits::RiskLimits;

#[test]
fn risk_blocks_large_notional() {
    let risk = SimpleRisk {
        limits: RiskLimits {
            max_notional: 100.0,
        },
    };
    let input = RiskInput {
        ts: Timestamp::new(0, TimeSource::Exchange),
        instrument: InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
        notional: 1000.0,
        side: Side::Buy,
    };
    assert!(matches!(risk.evaluate(&input), RiskVerdict::Block(_)));
}
