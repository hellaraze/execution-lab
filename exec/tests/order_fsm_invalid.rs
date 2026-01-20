use exec::order_fsm::OrderFsm;
use el_core::event::ExecEvent::*;
use exec::order_state::OrderState::*;

#[test]
fn reject_cannot_become_filled() {
    let mut fsm = OrderFsm::new();
    fsm.apply(OrderPlaced);
    fsm.apply(OrderRejected);
    fsm.apply(OrderFilled);
    assert_eq!(fsm.state, Rejected);
}
