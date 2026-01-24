use exec_engine::store::OrderStore;
use exec_engine::fsm::{OrderEvent, OrderState};

#[test]
fn crash_sim_replay_is_idempotent() {
    let mut s1 = OrderStore::new();
    s1.get_or_create(1, 100).unwrap();
    s1.apply(1, OrderEvent::Accept).unwrap();
    s1.apply(1, OrderEvent::Fill { fill_id: 1, qty_atoms: 10 }).unwrap();
    s1.apply(1, OrderEvent::Fill { fill_id: 2, qty_atoms: 20 }).unwrap();

    let snap = s1.export_snapshot(1).unwrap();
    assert_eq!(snap.filled_atoms, 30);

    let mut s2 = OrderStore::new();
    s2.import_snapshot(snap);

    let _ = s2.apply(1, OrderEvent::Accept);
    s2.apply(1, OrderEvent::Fill { fill_id: 1, qty_atoms: 10 }).unwrap();
    s2.apply(1, OrderEvent::Fill { fill_id: 2, qty_atoms: 20 }).unwrap();

    let o = s2.get_or_create(1, 100).unwrap();
    assert_eq!(o.data.filled_atoms, 30);
    assert_eq!(o.data.state, OrderState::Open);
}
