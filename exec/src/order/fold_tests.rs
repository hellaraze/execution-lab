#[cfg(test)]
mod tests {
    use super::super::events::OrderEvent;
    use super::fold_view;
    use super::super::types::OrderState;
    use el_core::event::ExecEvent::*;

    #[test]
    fn fold_happy_path_fill() {
        let events = vec![
            OrderEvent { ev: OrderPlaced },
            OrderEvent { ev: OrderAccepted },
            OrderEvent { ev: OrderPartiallyFilled },
            OrderEvent { ev: OrderFilled },
        ];
        let v = fold_view(&events).unwrap();
        assert_eq!(v.state, OrderState::Filled);
    }

    #[test]
    fn fold_cancel_after_partial() {
        let events = vec![
            OrderEvent { ev: OrderPlaced },
            OrderEvent { ev: OrderAccepted },
            OrderEvent { ev: OrderPartiallyFilled },
            OrderEvent { ev: OrderCanceled },
            OrderEvent { ev: OrderCanceled },
        ];
        let v = fold_view(&events).unwrap();
        assert_eq!(v.state, OrderState::Cancelled);
    }

    #[test]
    fn fold_terminal_rejects_more_events() {
        let events = vec![
            OrderEvent { ev: OrderPlaced },
            OrderEvent { ev: OrderAccepted },
            OrderEvent { ev: OrderFilled },
            OrderEvent { ev: OrderCanceled },
        ];
        assert!(fold_view(&events).is_err());
    }
}
