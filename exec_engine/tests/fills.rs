use exec_engine::store::OrderStore;
use exec_engine::fsm::OrderState;
use exec_engine::fsm::OrderEvent;
use exec_engine::error::ExecError;

#[test]
fn idempotent_fill() {
    let mut store = OrderStore::new();
    let o = store.get_or_create(1, 100);
    assert_eq!(o.data.state, OrderState::New);

    store.apply(1, OrderEvent::Accept).unwrap();

    store.apply(1, OrderEvent::Fill { fill_id: 1, qty_atoms: 50 }).unwrap();
    store.apply(1, OrderEvent::Fill { fill_id: 1, qty_atoms: 50 }).unwrap(); // replay

    let o = store.get_or_create(1, 100);
    assert_eq!(o.data.filled_atoms, 50);
}

#[test]
fn overfill_rejected() {
    let mut store = OrderStore::new();
    store.get_or_create(2, 100);
    store.apply(2, OrderEvent::Accept).unwrap();

    let err = store.apply(2, OrderEvent::Fill { fill_id: 1, qty_atoms: 150 });
    assert_eq!(err, Err(ExecError::Overfill));
}

#[test]
fn filled_then_no_more_fills() {
    let mut store = OrderStore::new();
    store.get_or_create(3, 100);
    store.apply(3, OrderEvent::Accept).unwrap();
    store.apply(3, OrderEvent::Fill { fill_id: 1, qty_atoms: 100 }).unwrap();

    let err = store.apply(3, OrderEvent::Fill { fill_id: 2, qty_atoms: 1 });
    assert_eq!(err, Err(ExecError::AlreadyFilled));
}
