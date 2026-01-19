use crate::events::{ExecEvent, OrderId};
use crate::order::OrderStore;

#[test]
fn store_builds_views_for_multiple_orders() {
    let a = OrderId(1);
    let b = OrderId(2);

    let events = vec![
        ExecEvent::OrderCreated { id: a },
        ExecEvent::OrderValidated { id: a },
        ExecEvent::OrderSent { id: a },
        ExecEvent::OrderAcked { id: a },

        ExecEvent::OrderCreated { id: b },
        ExecEvent::OrderValidated { id: b },
        ExecEvent::OrderSent { id: b },
        ExecEvent::OrderAcked { id: b },

        ExecEvent::OrderFill { id: a, filled_qty: 0.5, avg_px: 100.0 },
        ExecEvent::OrderCancelRequested { id: a },
        ExecEvent::OrderCancelled { id: a },
    ];

    let mut store = OrderStore::new();
    store.apply_all(&events).unwrap();

    assert_eq!(store.len(), 2);
    assert!(store.view(a).is_some());
    assert!(store.view(b).is_some());

    let va = store.view(a).unwrap();
    assert!(va.filled_qty > 0.0);

    let vb = store.view(b).unwrap();
    assert_eq!(vb.filled_qty, 0.0);
}
