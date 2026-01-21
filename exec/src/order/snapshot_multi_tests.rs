use crate::events::{ExecEvent, OrderId};
use crate::order::snapshot::build_snapshot_multi;
use crate::util::instrument::InstrumentKey;

#[test]
fn multi_snapshot_hash_is_deterministic() {
    let a = InstrumentKey::new("binance", "BTCUSDT");
    let b = InstrumentKey::new("binance", "ETHUSDT");

    let events = vec![
        ExecEvent::OrderCreated { instrument: a.clone(), id: OrderId(1) },
        ExecEvent::OrderAcked { instrument: a.clone(), id: OrderId(1) },
        ExecEvent::OrderCreated { instrument: b.clone(), id: OrderId(2) },
        ExecEvent::OrderAcked { instrument: b.clone(), id: OrderId(2) },
    ];

    let (_s1, h1) = build_snapshot_multi(&events).unwrap();
    let (_s2, h2) = build_snapshot_multi(&events).unwrap();

    assert_eq!(h1, h2);
}
