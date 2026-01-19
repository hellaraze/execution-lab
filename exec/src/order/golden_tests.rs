use crate::events::{ExecEvent, OrderId};
use crate::order::{apply, OrderView};
use crate::util::stable_hash_u64;

#[test]
fn golden_order_fsm_hash_is_stable() {
    let id = OrderId(42);

    let events = vec![
        ExecEvent::OrderCreated { id },
        ExecEvent::OrderValidated { id },
        ExecEvent::OrderSent { id },
        ExecEvent::OrderAcked { id },
        ExecEvent::OrderFill { id, filled_qty: 0.3, avg_px: 100.0 },
        ExecEvent::OrderFill { id, filled_qty: 0.7, avg_px: 101.0 },
        ExecEvent::OrderCancelRequested { id },
        ExecEvent::OrderCancelled { id },
    ];

    let mut view = OrderView::new();
    for ev in &events {
        apply(&mut view, id, ev).unwrap();
    }

    let bytes = serde_json::to_vec(&view).unwrap();
    let h = stable_hash_u64(&bytes);

    // NOTE: first time you run, we'll lock this value.
    // If it changes later, it means lifecycle semantics changed.
    assert_eq!(h, 7367865926763512830u64);
}
