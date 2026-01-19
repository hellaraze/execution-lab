use crate::events::{ExecEvent, OrderId};
use crate::order::build_snapshot;

#[test]
fn snapshot_hash_is_stable() {
    let a = OrderId(10);
    let b = OrderId(20);

    let events = vec![
        ExecEvent::OrderCreated { id: a },
        ExecEvent::OrderValidated { id: a },
        ExecEvent::OrderSent { id: a },
        ExecEvent::OrderAcked { id: a },
        ExecEvent::OrderFill { id: a, filled_qty: 1.0, avg_px: 100.0 },

        ExecEvent::OrderCreated { id: b },
        ExecEvent::OrderValidated { id: b },
        ExecEvent::OrderSent { id: b },
        ExecEvent::OrderAcked { id: b },
        ExecEvent::OrderCancelRequested { id: b },
        ExecEvent::OrderCancelled { id: b },
    ];

    let (_store, h) = build_snapshot(&events).unwrap();

    // First run: we will lock this value.
    assert_eq!(h, 9070264859527619342u64);
}
