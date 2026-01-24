use exec_engine::fsm::OrderEvent;
use exec_engine::store::OrderStore;
use std::fs;

#[test]
fn e2e_snapshot_envelope_file_roundtrip_hash() {
    // build state
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

    // export envelope -> json file
    let env = s1.export_envelope(1).unwrap();
    let json = serde_json::to_string_pretty(&env).unwrap();

    let path = std::env::temp_dir().join("exec_engine_snapshot_env.json");
    fs::write(&path, &json).unwrap();

    // "restart": new store loads json -> envelope -> import
    let json2 = fs::read_to_string(&path).unwrap();
    let env2: exec_engine::store::snapshot::SnapshotEnvelope =
        serde_json::from_str(&json2).unwrap();

    let mut s2 = OrderStore::new();
    s2.import_envelope(env2).unwrap();

    let h2 = s2.snapshot_hash_hex(1).unwrap();

    assert_eq!(h1, h2);

    // cleanup
    let _ = fs::remove_file(&path);
}
