use exec::events::{ExecEvent, OrderId};
use exec::order::snapshot::build_snapshot;
use exec::util::instrument::InstrumentKey;

#[test]
fn snapshot_is_deterministic_even_if_stream_has_terminal_then_noise() {
    let btc = InstrumentKey::new("binance", "BTCUSDT");

    // terminal event then extra noise (can happen in real logs / reorder / retries)
    let events = vec![
        ExecEvent::OrderCreated {
            instrument: btc.clone(),
            id: OrderId(1),
        },
        ExecEvent::OrderCancelled {
            instrument: btc.clone(),
            id: OrderId(1),
        },
        // "noise" after terminal: the system should remain deterministic and safe
        ExecEvent::OrderAcked {
            instrument: btc.clone(),
            id: OrderId(1),
        },
    ];

    let (_s1, h1) = build_snapshot(&events).expect("snapshot ok");
    let (_s2, h2) = build_snapshot(&events).expect("snapshot ok");
    assert_eq!(h1, h2, "snapshot hash must be deterministic");
}
