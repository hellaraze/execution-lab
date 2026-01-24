use exec_engine::store::OrderStore;
use exec_engine::fsm::OrderEvent;

#[test]
fn snapshot_envelope_roundtrip_is_stable() {
    let mut s1 = OrderStore::new();
    s1.get_or_create(1, 100).unwrap();
    s1.apply(1, OrderEvent::Accept).unwrap();
    s1.apply(1, OrderEvent::Fill { fill_id: 1, qty_atoms: 10 }).unwrap();
    s1.apply(1, OrderEvent::Fill { fill_id: 2, qty_atoms: 20 }).unwrap();

    let h1 = s1.snapshot_hash_hex(1).unwrap();

    let env = s1.export_envelope(1).unwrap();
    let json = serde_json::to_string_pretty(&env).unwrap();

    let env2: exec_engine::store::snapshot::SnapshotEnvelope = serde_json::from_str(&json).unwrap();

    let mut s2 = OrderStore::new();
    s2.import_envelope(env2).unwrap();

    let h2 = s2.snapshot_hash_hex(1).unwrap();

    assert_eq!(h1, h2);
}
