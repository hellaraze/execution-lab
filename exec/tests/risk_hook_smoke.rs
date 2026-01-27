use el_core::event::Exchange;
use el_core::instrument::InstrumentKey;
use el_core::time::{TimeSource, Timestamp};
use el_obs::sink::NoopSink;
use el_risk::contract::*;
use el_risk::decision::SimpleRisk;
use el_risk::limits::RiskLimits;
use exec::risk_hook::risk_precheck;

#[test]
fn risk_hook_blocks_and_emits() {
    let risk = SimpleRisk {
        limits: RiskLimits { max_notional: 10.0 },
    };
    let mut sink = NoopSink;

    let input = RiskInput {
        ts: Timestamp::new(0, TimeSource::Exchange),
        instrument: InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
        notional: 1000.0,
        side: Side::Buy,
    };

    let v = risk_precheck(&risk, &mut sink, &input);
    assert!(matches!(v, RiskVerdict::Block(_)));
}
