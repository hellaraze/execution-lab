use el_core::event::Exchange;
use el_core::instrument::InstrumentKey;
use exec::adapter::{ExecAdapter, PlaceOrder, Side};
use exec::events::OrderId;
use exec::mock_adapter::MockAdapter;

#[test]
fn mock_adapter_accepts_order() {
    let mut adapter = MockAdapter;

    let res = adapter.place_order(PlaceOrder {
        instrument: InstrumentKey::new(Exchange::Binance, "BTCUSDT"),
        order_id: OrderId(1),
        price: 100.0,
        qty: 1.0,
        side: Side::Buy,
    });

    assert!(matches!(res, exec::adapter::ExecResult::Accepted));
}
