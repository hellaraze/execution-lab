use std::fs;

use serde_json::Value;

fn normalize_json(v: &mut Value) {
    match v {
        Value::Object(m) => {
            let mut keys: Vec<String> = m.keys().cloned().collect();
            keys.sort();
            let mut new = serde_json::Map::new();
            for k in keys {
                let mut vv = m.remove(&k).unwrap();
                normalize_json(&mut vv);
                new.insert(k, vv);
            }
            *m = new;
        }
        Value::Array(a) => {
            for x in a.iter_mut() {
                normalize_json(x);
            }
        }
        _ => {}
    }
}

#[test]
fn schema_snapshot_envelope_golden() {
    use exec_engine::fsm::OrderState;
    use exec_engine::store::snapshot::{OrderSnapshot, SnapshotEnvelope};

    // Canonical empty-ish snapshot (stable JSON baseline)
    let order = OrderSnapshot {
        id: 0,
        state: OrderState::New,
        total_atoms: 0,
        filled_atoms: 0,
        fill_qty: Vec::new(),
    };

    let env = SnapshotEnvelope::v1(order);

    let mut v = serde_json::to_value(&env).expect("to_value");
    normalize_json(&mut v);
    let s = serde_json::to_string_pretty(&v).expect("to_string_pretty");

    let path = format!(
        "{}/tests/data/schema_snapshot_envelope.json",
        env!("CARGO_MANIFEST_DIR")
    );

    if std::env::var("EL_UPDATE_GOLDEN").as_deref() == Ok("1") {
        let dir = std::path::Path::new(&path).parent().unwrap();
        fs::create_dir_all(dir).expect("create golden dir");
        fs::write(&path, &s).expect("write golden");
        return;
    }

    let cur = fs::read_to_string(&path).expect("read golden (set EL_UPDATE_GOLDEN=1 to regen)");
    assert_eq!(
        cur, s,
        "schema mismatch: run `EL_UPDATE_GOLDEN=1 cargo test -q -p exec_engine --test schema_snapshot_envelope schema_snapshot_envelope_golden` and commit updated golden ONLY if intentional"
    );
}
