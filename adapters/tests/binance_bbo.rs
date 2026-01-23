use adapters::contracts::BinanceMdAdapterBbo;
use el_contracts::v1::md;
use el_contracts::v1::{MarketDataAdapter, MdEvent};
use el_core::event::Exchange;
use el_core::instrument::InstrumentKey;

#[test]
fn binance_bookticker_maps_to_bbo() {
    let instrument = InstrumentKey::new(Exchange::Binance, "BTCUSDT");
    let mut a = BinanceMdAdapterBbo::new(instrument.clone());

    // Binance bookTicker payload example (compact form)
    let raw = r#"{"u":400900217,"s":"BTCUSDT","b":"92499.29","B":"4.27501","a":"92499.30","A":"0.54029"}"#;

    a.push_raw(raw, 1, 1700000000000);
    let out = MarketDataAdapter::poll(&mut a);
    assert_eq!(out.len(), 1);

    match out[0].clone() {
        MdEvent::Bbo(md::Bbo {
            instrument: i,
            bid_px,
            bid_qty,
            ask_px,
            ask_qty,
            ..
        }) => {
            assert_eq!(i, instrument);
            assert!((bid_px - 92499.29).abs() < 1e-9);
            assert!((bid_qty - 4.27501).abs() < 1e-9);
            assert!((ask_px - 92499.30).abs() < 1e-9);
            assert!((ask_qty - 0.54029).abs() < 1e-9);
        }
        _ => panic!("expected MdEvent::Bbo"),
    }
}
