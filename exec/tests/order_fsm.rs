use exec::order_fsm::OrderFsm;
use el_core::event::ExecEvent::*;
use exec::order_state::OrderState::*;

#[test]
fn happy_path_fill() {
    let mut fsm = OrderFsm::new();
    fsm.apply(OrderPlaced);
    fsm.apply(OrderAccepted);
    fsm.apply(OrderPartiallyFilled);
    fsm.apply(OrderFilled);
    assert_eq!(fsm.state, Filled);
}

#[test]
fn cancel_after_partial() {
    let mut fsm = OrderFsm::new();
    fsm.apply(OrderPlaced);
    fsm.apply(OrderAccepted);
    fsm.apply(OrderPartiallyFilled);
    fsm.apply(OrderCanceled);
    assert_eq!(fsm.state, Canceled);
}
