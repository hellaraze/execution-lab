use exec_engine::fsm::{apply, OrderData, OrderEvent, OrderState};

#[test]
fn basic_flow() {
    let mut d = OrderData::new(100);
    assert_eq!(d.state, OrderState::New);

    apply(&mut d, &OrderEvent::Accept).unwrap();
    assert_eq!(d.state, OrderState::Open);

    apply(&mut d, &OrderEvent::Fill { fill_id: 1, qty_atoms: 40 }).unwrap();
    assert_eq!(d.filled_atoms, 40);
    assert_eq!(d.state, OrderState::Open);

    apply(&mut d, &OrderEvent::Fill { fill_id: 2, qty_atoms: 60 }).unwrap();
    assert_eq!(d.filled_atoms, 100);
    assert_eq!(d.state, OrderState::Filled);
}
