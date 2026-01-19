use anyhow::{Context, Result};
use el_core::event::{Event, EventPayload, EventType};
use eventlog::EventLogReader;
use orderbook::OrderBook;

fn run_and_collect_hashes(path: &str, max_events: usize) -> Result<Vec<u64>> {
    let mut r = EventLogReader::open(path).with_context(|| format!("open log: {}", path))?;

    let mut book = OrderBook::new();
    let mut out: Vec<u64> = Vec::new();
    let mut n: usize = 0;

    while let Some((env, payload_bytes)) = r.next()? {
        let ev: Event = serde_json::from_slice(&payload_bytes)
            .with_context(|| format!("parse core::Event json (seq={})", env.seq))?;

        match (&ev.event_type, &ev.payload) {
            (EventType::BookSnapshot, EventPayload::BookSnapshot { bids, asks }) => {
                book = OrderBook::new();
                book.apply_levels(&bids, &asks);
            }
            (EventType::BookDelta, EventPayload::BookDelta { bids, asks }) => {
                book.apply_levels(&bids, &asks);
            }
            _ => {}
        }

        book.check_invariants()
            .map_err(|e| anyhow::anyhow!("invariant fail at n={}: {}", n + 1, e))?;

        out.push(book.state_hash64());
        n += 1;

        if n >= max_events {
            break;
        }
    }

    Ok(out)
}

fn parse_expected(path: &str) -> Result<Vec<u64>> {
    let s = std::fs::read_to_string(path).with_context(|| format!("read {}", path))?;
    let mut v = Vec::new();
    for (i, line) in s.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let x: u64 = line
            .parse()
            .with_context(|| format!("parse u64 at line {}", i + 1))?;
        v.push(x);
    }
    Ok(v)
}

#[test]
fn golden_replay_hashes_match() -> Result<()> {
    let log_path = "tests/data/golden_events_book.log";
    let expected_path = "tests/golden_hashes.txt";

    let expected = parse_expected(expected_path)?;
    let actual = run_and_collect_hashes(log_path, expected.len())?;

    anyhow::ensure!(
        expected == actual,
        "golden mismatch: expected {} hashes, got {}",
        expected.len(),
        actual.len()
    );

    Ok(())
}

#[test]
fn replay_is_deterministic_double_run() -> Result<()> {
    let log_path = "tests/data/golden_events_book.log";
    let steps = 200;

    let h1 = run_and_collect_hashes(log_path, steps)?;
    let h2 = run_and_collect_hashes(log_path, steps)?;

    anyhow::ensure!(h1 == h2, "replay is NOT deterministic");
    Ok(())
}
