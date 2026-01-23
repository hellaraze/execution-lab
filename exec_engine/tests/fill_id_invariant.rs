use exec_engine::store::OrderStore;
use exec_engine::fsm::OrderEvent;
use exec_engine::error::ExecError;

#[test]
fn same_fill_id_must_match_qty() {
    let mut store = OrderStore::new();
    store.get_or_create(1, 100).unwrap();
    store.apply(1, OrderEvent::Accept).unwrap();

    store.apply(1, OrderEvent::Fill { fill_id: 7, qty_atoms: 10 }).unwrap();

    let err = store.apply(1, OrderEvent::Fill { fill_id: 7, qty_atoms: 11 });
    assert_eq!(err, Err(ExecError::InvalidTransition));
}
