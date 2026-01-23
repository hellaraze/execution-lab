use adapters::contracts::BinanceMdMuxAdapterBbo;
use el_contracts::v1::{MarketDataAdapter, MdEvent};
use el_core::event::Exchange;
use el_core::instrument::InstrumentKey;

#[test]
fn mux_emits_multi_instrument_bbo_and_dedups_per_symbol() {
    let mut m = BinanceMdMuxAdapterBbo::default();

    let raw_btc = r#"{"u":1,"s":"BTCUSDT","b":"100.0","B":"1.0","a":"101.0","A":"2.0"}"#;
    let raw_eth = r#"{"u":1,"s":"ETHUSDT","b":"10.0","B":"3.0","a":"10.5","A":"4.0"}"#;

    m.push_raw(raw_btc, 1, 1700000000000);
    m.push_raw(raw_eth, 1, 1700000000001);

    // dup (should be ignored for BTC only)
    m.push_raw(raw_btc, 1, 1700000000002);

    let out = MarketDataAdapter::poll(&mut m);
    assert_eq!(out.len(), 2);

    let want_btc = InstrumentKey::new(Exchange::Binance, "BTCUSDT");
    let want_eth = InstrumentKey::new(Exchange::Binance, "ETHUSDT");

    let mut got = out
        .into_iter()
        .map(|e| match e {
            MdEvent::Bbo(b) => b.instrument,
            _ => panic!("expected Bbo"),
        })
        .collect::<Vec<_>>();

    got.sort_by(|a, b| a.to_string().cmp(&b.to_string()));

    let mut want = vec![want_btc, want_eth];
    want.sort_by(|a, b| a.to_string().cmp(&b.to_string()));

    assert_eq!(got, want);
}
