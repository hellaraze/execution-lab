use std::fs;

use serde_json::Value;

use exec::events::ExecEvent;
use exec::events::OrderId;
use exec::util::instrument::InstrumentKey;

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
fn schema_exec_event_golden() {
    let btc = InstrumentKey::new("binance", "BTCUSDT");

    let samples = vec![
        ExecEvent::OrderCreated {
            instrument: btc.clone(),
            id: OrderId(1),
        },
        ExecEvent::OrderAcked {
            instrument: btc.clone(),
            id: OrderId(1),
        },
        ExecEvent::OrderCancelled {
            instrument: btc.clone(),
            id: OrderId(1),
        },
        ExecEvent::OrderExpired {
            instrument: btc.clone(),
            id: OrderId(1),
        },
    ];

    let mut out = Vec::new();
    for ev in samples {
        let mut v = serde_json::to_value(&ev).expect("to_value");
        normalize_json(&mut v);
        out.push(v);
    }

    let s = serde_json::to_string_pretty(&out).expect("to_string_pretty");
    let path = format!(
        "{}/tests/data/schema_exec_event.json",
        env!("CARGO_MANIFEST_DIR")
    );
    if std::env::var("EL_UPDATE_GOLDEN").as_deref() == Ok("1") {
        let dir = std::path::Path::new(&path).parent().unwrap();
        fs::create_dir_all(dir).expect("create golden dir");
        fs::write(&path, &s).expect("write golden");
        return;
    }

    let cur = fs::read_to_string(&path).expect("read golden (set EL_UPDATE_GOLDEN=1 to regen)");
    assert_eq!(cur, s, "schema mismatch: run `EL_UPDATE_GOLDEN=1 cargo test -q -p exec schema_exec_event_golden` and commit the updated golden ONLY if intentional");
}
