use anyhow::{Context, Result};

mod util;

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
fn golden_replay_chain_hashes_match() -> Result<()> {
    let log_path = "tests/data/golden_events_book.log";
    let expected_path = "tests/golden_hashes.txt";

    let expected = parse_expected(expected_path)?;
    let actual = util::run_and_collect_chain_hashes(log_path, expected.len())?;

    anyhow::ensure!(expected == actual, "GOLDEN MISMATCH");
    Ok(())
}

#[test]
fn replay_is_deterministic_double_run() -> Result<()> {
    let log_path = "tests/data/golden_events_book.log";
    let steps = 200;

    let h1 = util::run_and_collect_chain_hashes(log_path, steps)?;
    let h2 = util::run_and_collect_chain_hashes(log_path, steps)?;
    anyhow::ensure!(h1 == h2, "REPLAY NOT DETERMINISTIC");

    Ok(())
}
