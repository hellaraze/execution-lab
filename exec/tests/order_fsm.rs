use el_core::event::ExecEvent::*;
use exec::order_fsm::OrderFsm;
use exec::order_state::OrderState::*;

#[test]
fn happy_path_fill() {
    let mut fsm = OrderFsm::new();
    fsm.apply(OrderPlaced).unwrap();
    fsm.apply(OrderAccepted).unwrap();
    fsm.apply(OrderPartiallyFilled).unwrap();
    fsm.apply(OrderFilled).unwrap();
    assert_eq!(fsm.state, Filled);
}

#[test]
fn cancel_after_partial() {
    let mut fsm = OrderFsm::new();
    fsm.apply(OrderPlaced).unwrap();
    fsm.apply(OrderAccepted).unwrap();
    fsm.apply(OrderPartiallyFilled).unwrap();
    fsm.apply(OrderCanceled).unwrap();
    assert_eq!(fsm.state, Canceled);
}
