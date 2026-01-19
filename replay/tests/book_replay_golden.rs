use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use orderbook::OrderBook;
use replay::wire::BookLevels;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root")
        .to_path_buf()
}

fn open_repo_file(rel: &str) -> File {
    let p = repo_root().join(rel);
    File::open(&p).unwrap_or_else(|e| panic!("open file {:?}: {}", p, e))
}

fn read_first_line_repo(rel: &str) -> String {
    let f = open_repo_file(rel);
    let mut r = BufReader::new(f);
    let mut s = String::new();
    r.read_line(&mut s).expect("read line");
    s
}

fn extract_str<'a>(v: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    v.get(key)?.as_str()
}

fn extract_seq(v: &serde_json::Value) -> Option<u64> {
    v.get("seq")?.as_u64()
}

#[test]
fn replay_book_snapshot_plus_events_log_deltas_golden_hash() {
    // --- snapshot from events_book.log ---
    let snap_line = read_first_line_repo("events_book.log");
    let snap_v: serde_json::Value =
        serde_json::from_str(snap_line.trim()).expect("parse snapshot json");

    assert_eq!(extract_str(&snap_v, "event_type"), Some("BookSnapshot"));
    assert_eq!(extract_str(&snap_v, "exchange"), Some("Binance"));
    assert_eq!(extract_str(&snap_v, "symbol"), Some("BTCUSDT"));

    let snap_seq = extract_seq(&snap_v).expect("snapshot seq");

    let payload = snap_v.get("payload").expect("payload");
    let snap_payload = payload.get("BookSnapshot").expect("BookSnapshot payload");
    let levels: BookLevels =
        serde_json::from_value(snap_payload.clone()).expect("parse snapshot levels");

    let mut book = OrderBook::new();
    book.apply_levels(&levels.bids, &levels.asks);

    // --- deltas from events.log AFTER snapshot seq (if any) ---
    let f = open_repo_file("events.log");
    let r = BufReader::new(f);

    let mut applied_deltas = 0usize;
    let max_deltas: usize = 5000;

    for line in r.lines() {
        let line = line.expect("read line");
        let t = line.trim();
        if t.is_empty() {
            continue;
        }

        let v: serde_json::Value = match serde_json::from_str(t) {
            Ok(x) => x,
            Err(_) => continue,
        };

        if extract_str(&v, "exchange") != Some("Binance") {
            continue;
        }
        if extract_str(&v, "symbol") != Some("BTCUSDT") {
            continue;
        }
        if extract_str(&v, "event_type") != Some("BookDelta") {
            continue;
        }

        // Only deltas strictly after snapshot seq when seq is present
        let seq = match extract_seq(&v) {
            Some(s) => s,
            None => continue,
        };
        if seq <= snap_seq {
            continue;
        }

        let payload = v.get("payload").expect("payload");
        let delta_payload = payload.get("BookDelta").expect("BookDelta payload");
        let levels: BookLevels =
            serde_json::from_value(delta_payload.clone()).expect("parse delta levels");

        book.apply_levels(&levels.bids, &levels.asks);

        applied_deltas += 1;
        if applied_deltas >= max_deltas {
            break;
        }
    }

    book.check_invariants().expect("invariants");

    let h = book.state_hash64();

    // Golden (snapshot + up to 5000 deltas after it, if present in events.log)
    assert_eq!(h, 11732371981392004380u64);

    let _ = applied_deltas;
}
