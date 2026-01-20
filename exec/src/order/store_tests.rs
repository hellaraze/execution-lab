#[cfg(test)]
mod tests {
    use crate::events::{ExecEvent, OrderId};
    use crate::order::{build_snapshot, OrderState};

    #[test]
    fn snapshot_builds_and_hashes_deterministically() {
        let id = OrderId(1);

        let events = vec![
            ExecEvent::OrderCreated { id },
            ExecEvent::OrderValidated { id },
            ExecEvent::OrderSent { id },
            ExecEvent::OrderAcked { id },
            ExecEvent::OrderFill { id, filled_qty: 1.0, avg_px: 100.0 },
            ExecEvent::OrderFill { id, filled_qty: 3.0, avg_px: 106.6666666667 },
            ExecEvent::OrderCancelRequested { id },
            ExecEvent::OrderCancelled { id },
        ];

        let (_store1, h1) = build_snapshot(&events).unwrap();
        let (_store2, h2) = build_snapshot(&events).unwrap();

        assert_eq!(h1, h2, "snapshot hash must be deterministic");
    }

    #[test]
    fn snapshot_view_state_is_terminal() {
        let id = OrderId(7);

        let events = vec![
            ExecEvent::OrderCreated { id },
            ExecEvent::OrderSent { id },
            ExecEvent::OrderAcked { id },
            ExecEvent::OrderFill { id, filled_qty: 2.0, avg_px: 50.0 },
            ExecEvent::OrderCancelled { id },
        ];

        let (store, _h) = build_snapshot(&events).unwrap();
        let view = store.view(id).unwrap();
        assert_eq!(view.state, OrderState::Cancelled);
        assert!(view.filled_qty >= 0.0);
        assert!(view.avg_px >= 0.0);
    }
}
