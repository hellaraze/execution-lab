use exec_engine::error::ExecError;
use exec_engine::fsm::OrderEvent;
use exec_engine::store::OrderStore;

#[test]
fn apply_unknown_order_is_not_found() {
    let mut store = OrderStore::new();
    let err = store.apply(999, OrderEvent::Accept);
    assert_eq!(err, Err(ExecError::NotFound));
}

#[test]
fn total_atoms_mismatch_is_error() {
    let mut store = OrderStore::new();
    store.get_or_create(1, 100).unwrap();
    let err = store.get_or_create(1, 101).map(|_| ());
    assert_eq!(err, Err(ExecError::ConfigMismatch));
}
