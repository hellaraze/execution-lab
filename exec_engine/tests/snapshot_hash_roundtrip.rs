use exec_engine::fsm::OrderEvent;
use exec_engine::store::OrderStore;

#[test]
fn snapshot_hash_roundtrip_is_stable() {
    let mut s1 = OrderStore::new();
    s1.get_or_create(1, 100).unwrap();
    s1.apply(1, OrderEvent::Accept).unwrap();
    s1.apply(
        1,
        OrderEvent::Fill {
            fill_id: 1,
            qty_atoms: 10,
        },
    )
    .unwrap();
    s1.apply(
        1,
        OrderEvent::Fill {
            fill_id: 2,
            qty_atoms: 20,
        },
    )
    .unwrap();

    let h1 = s1.snapshot_hash_hex(1).unwrap();

    let snap = s1.export_snapshot(1).unwrap();
    let json = serde_json::to_string_pretty(&snap).unwrap();

    let snap2: exec_engine::store::snapshot::OrderSnapshot = serde_json::from_str(&json).unwrap();

    let mut s2 = OrderStore::new();
    s2.import_snapshot(snap2);

    let h2 = s2.snapshot_hash_hex(1).unwrap();

    assert_eq!(h1, h2);
}
