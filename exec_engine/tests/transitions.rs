use exec_engine::fsm::{apply, OrderData, OrderEvent, OrderState};
use exec_engine::error::ExecError;

#[test]
fn accept_only_from_new() {
    let mut d = OrderData::new(10);
    apply(&mut d, &OrderEvent::Accept).unwrap();
    assert_eq!(d.state, OrderState::Open);

    let err = apply(&mut d, &OrderEvent::Accept);
    assert_eq!(err, Err(ExecError::InvalidTransition));
}

#[test]
fn reject_only_from_new() {
    let mut d = OrderData::new(10);
    apply(&mut d, &OrderEvent::Reject).unwrap();
    assert_eq!(d.state, OrderState::Rejected);

    let mut d2 = OrderData::new(10);
    apply(&mut d2, &OrderEvent::Accept).unwrap();
    let err = apply(&mut d2, &OrderEvent::Reject);
    assert_eq!(err, Err(ExecError::InvalidTransition));
}

#[test]
fn cancel_only_from_new_or_open() {
    let mut d = OrderData::new(10);
    apply(&mut d, &OrderEvent::Cancel).unwrap();
    assert_eq!(d.state, OrderState::Canceled);

    let mut d2 = OrderData::new(10);
    apply(&mut d2, &OrderEvent::Accept).unwrap();
    apply(&mut d2, &OrderEvent::Cancel).unwrap();
    assert_eq!(d2.state, OrderState::Canceled);

    let mut d3 = OrderData::new(10);
    apply(&mut d3, &OrderEvent::Accept).unwrap();
    apply(&mut d3, &OrderEvent::Fill { fill_id: 1, qty_atoms: 10 }).unwrap();
    assert_eq!(d3.state, OrderState::Filled);
    let err = apply(&mut d3, &OrderEvent::Cancel);
    assert_eq!(err, Err(ExecError::InvalidTransition));
}
