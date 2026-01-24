use exec_engine::fsm::OrderEvent;
use exec_engine::store::OrderStore;

#[test]
fn golden_snapshot_json() {
    let mut s = OrderStore::new();
    s.get_or_create(1, 100).unwrap();
    s.apply(1, OrderEvent::Accept).unwrap();
    s.apply(
        1,
        OrderEvent::Fill {
            fill_id: 2,
            qty_atoms: 20,
        },
    )
    .unwrap();
    s.apply(
        1,
        OrderEvent::Fill {
            fill_id: 1,
            qty_atoms: 10,
        },
    )
    .unwrap();

    let snap = s.export_snapshot(1).unwrap();
    let json = serde_json::to_string_pretty(&snap).unwrap();

    let golden = include_str!("data/golden_snapshot.json");
    assert_eq!(json.trim(), golden.trim());
}
