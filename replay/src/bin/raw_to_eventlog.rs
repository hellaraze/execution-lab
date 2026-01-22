use anyhow::{Context, Result};
use eventlog::writer::Durability;
use eventlog::EventLogWriter;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let in_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "events_book.log".to_string());
    let out_path = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "events_book.eventlog".to_string());

    let f = File::open(&in_path).with_context(|| format!("open raw {}", in_path))?;
    let r = BufReader::new(f);

    let mut w = EventLogWriter::open_append(&out_path, "raw", Durability::Buffered)
        .with_context(|| format!("open eventlog {}", out_path))?;

    let mut n: u64 = 0;

    for line in r.lines() {
        let s = line?;
        if s.trim().is_empty() {
            continue;
        }

        let mut v: serde_json::Value = serde_json::from_str(&s)
            .with_context(|| format!("parse raw json line {}", n + 1))?;

        // legacy raw: inject `instrument` if missing
        if v.get("instrument").is_none() {
            let exchange = v.get("exchange").cloned().unwrap_or(serde_json::Value::Null);
            let symbol = v.get("symbol").cloned().unwrap_or(serde_json::Value::Null);
            v["instrument"] = serde_json::json!({ "exchange": exchange, "symbol": symbol });
        }

        w.append_json_value("event", 0, &v)
            .with_context(|| format!("write eventlog line {}", n + 1))?;

        n += 1;
    }

    w.flush().context("flush")?;
    println!("raw_to_eventlog ok: in={} out={} n={}", in_path, out_path, n);
    Ok(())
}
