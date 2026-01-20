#[cfg(test)]
mod tests {
    use super::super::events::OrderEvent;
    use super::fold_view;
    use super::super::types::OrderState;
    use el_core::event::{EventPayload, EventType};

    fn ev(event_type: EventType, payload: EventPayload) -> OrderEvent {
        OrderEvent { event_type, payload }
    }

    #[test]
    fn fills_accounting_and_avg_px() {
        let order_qty = 3.0;

        let events = vec![
            ev(EventType::OrderSubmit, EventPayload::OrderSubmit {
                order_id: "o1".into(),
                side: "Buy".into(),
                price: 100.0,
                qty: order_qty,
            }),
            ev(EventType::OrderAck, EventPayload::OrderAck { order_id: "o1".into() }),
            ev(EventType::Fill, EventPayload::Fill {
                order_id: "o1".into(),
                fill_id: "f1".into(),
                price: 100.0,
                qty: 1.0,
            }),
            ev(EventType::Fill, EventPayload::Fill {
                order_id: "o1".into(),
                fill_id: "f2".into(),
                price: 110.0,
                qty: 2.0,
            }),
        ];

        let v = fold_view(&events, order_qty).unwrap();
        assert_eq!(v.state, OrderState::Filled);
        assert!((v.filled_qty - 3.0).abs() < 1e-12);
        // avg = (1*100 + 2*110)/3 = 320/3
        assert!((v.avg_px - (320.0/3.0)).abs() < 1e-12);
    }

    #[test]
    fn duplicate_fill_id_is_idempotent() {
        let order_qty = 1.0;

        let events = vec![
            ev(EventType::OrderSubmit, EventPayload::OrderSubmit {
                order_id: "o1".into(),
                side: "Buy".into(),
                price: 100.0,
                qty: order_qty,
            }),
            ev(EventType::OrderAck, EventPayload::OrderAck { order_id: "o1".into() }),
            ev(EventType::Fill, EventPayload::Fill {
                order_id: "o1".into(),
                fill_id: "f1".into(),
                price: 100.0,
                qty: 1.0,
            }),
            // duplicate same fill_id should not overfill
            ev(EventType::Fill, EventPayload::Fill {
                order_id: "o1".into(),
                fill_id: "f1".into(),
                price: 100.0,
                qty: 1.0,
            }),
        ];

        let v = fold_view(&events, order_qty).unwrap();
        assert_eq!(v.state, OrderState::Filled);
        assert!((v.filled_qty - 1.0).abs() < 1e-12);
    }

    #[test]
    fn overfill_is_error() {
        let order_qty = 1.0;

        let events = vec![
            ev(EventType::OrderSubmit, EventPayload::OrderSubmit {
                order_id: "o1".into(),
                side: "Buy".into(),
                price: 100.0,
                qty: order_qty,
            }),
            ev(EventType::OrderAck, EventPayload::OrderAck { order_id: "o1".into() }),
            ev(EventType::Fill, EventPayload::Fill {
                order_id: "o1".into(),
                fill_id: "f1".into(),
                price: 100.0,
                qty: 2.0,
            }),
        ];

        assert!(fold_view(&events, order_qty).is_err());
    }

    #[test]
    fn terminal_state_rejects_more_events() {
        let order_qty = 1.0;

        let events = vec![
            ev(EventType::OrderSubmit, EventPayload::OrderSubmit {
                order_id: "o1".into(),
                side: "Buy".into(),
                price: 100.0,
                qty: order_qty,
            }),
            ev(EventType::OrderAck, EventPayload::OrderAck { order_id: "o1".into() }),
            ev(EventType::Fill, EventPayload::Fill {
                order_id: "o1".into(),
                fill_id: "f1".into(),
                price: 100.0,
                qty: 1.0,
            }),
            ev(EventType::CancelRequest, EventPayload::CancelRequest { order_id: "o1".into() }),
        ];

        assert!(fold_view(&events, order_qty).is_err());
    }
}
