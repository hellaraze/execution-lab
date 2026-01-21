use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

use el_core::event::Event;
use uuid::Uuid;

use exec::order::{build_snapshot, to_exec_event};

fn main() -> Result<()> {
    let path = "exec/tests/data/core_exec_golden.jsonl";
    let f = File::open(path).with_context(|| format!("open {}", path))?;
    let r = BufReader::new(f);

    let mut exec_events = Vec::new();

    for line in r.lines() {
        let line = line?;
        let mut v: serde_json::Value =
            serde_json::from_str(&line).with_context(|| "parse json")?;

        // deterministic id injection
        if v.get("id").is_none() {
            let raw = serde_json::to_vec(&v).expect("json to vec");
            let h = blake3::hash(&raw);
            let mut b = [0u8; 16];
            b.copy_from_slice(&h.as_bytes()[0..16]);
            v["id"] = serde_json::Value::String(Uuid::from_bytes(b).to_string());
        }

        let ev: Event =
            serde_json::from_value(v).with_context(|| "parse core Event")?;

        if let Some(x) = to_exec_event(&ev)? {
            exec_events.push(x);
        }
    }

    let (_store, h) = build_snapshot(&exec_events)?;
    println!("{h}");
    Ok(())
}
