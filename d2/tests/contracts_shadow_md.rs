use d2::contracts_shadow::shadow_md_event_from_bbo;
use el_core::instrument::InstrumentKey;
use el_core::time::{TimeSource, Timestamp};

#[test]
fn shadow_md_event_builds() {
    let instrument = {
        use el_core::event::Exchange;
        InstrumentKey::new(Exchange::Binance, "BTCUSDT")
    };
    let ts = Timestamp::new(123, TimeSource::Process);

    let ev = shadow_md_event_from_bbo(instrument.clone(), ts, 1.0, 2.0, 3.0, 4.0);

    assert_eq!(ev.instrument, instrument);
    assert_eq!(ev.ts, ts);
    assert_eq!(ev.bbo.bid_px, 1.0);
    assert_eq!(ev.bbo.bid_qty, 2.0);
    assert_eq!(ev.bbo.ask_px, 3.0);
    assert_eq!(ev.bbo.ask_qty, 4.0);
}
