use el_core::event::ExecEvent::*;
use exec::order_fsm::OrderFsm;
use exec::order_state::OrderState::*;

#[test]
fn reject_cannot_become_filled() {
    let mut fsm = OrderFsm::new();
    fsm.apply(OrderPlaced).unwrap();
    fsm.apply(OrderRejected).unwrap();
    assert!(fsm.apply(OrderFilled).is_err());
    assert_eq!(fsm.state, Rejected);
}
