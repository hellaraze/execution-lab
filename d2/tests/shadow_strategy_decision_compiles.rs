use contracts_bridge::v1 as c1;
use d2::contracts_shadow::shadow_strategy_decision;
use el_core::event::Exchange;
use el_core::instrument::InstrumentKey;
use el_core::time::{TimeSource, Timestamp};

#[test]
fn shadow_strategy_decision_builds() {
    let instrument = InstrumentKey::new(Exchange::Binance, "BTCUSDT");
    let ts = Timestamp::new(123, TimeSource::Process);
    let ev = shadow_strategy_decision(instrument.clone(), ts, c1::strategy::Decision::Gas, 12.34);

    assert_eq!(ev.instrument, instrument);
    assert_eq!(ev.ts, ts);
    assert_eq!(ev.decision, c1::strategy::Decision::Gas);
    assert_eq!(ev.edge_bps, 12.34);
}
