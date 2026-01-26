use anyhow::{Context, Result};
use eventlog::writer::Durability;
use eventlog::EventLogWriter;
use serde_json::{json, Value};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn usage() -> ! {
    eprintln!("usage: cargo run -q -p replay --bin raw_to_eventlog -- <in_raw.jsonl> <out.eventlog> [stream] [kind]");
    std::process::exit(2);
}

fn extract_ts_ns(v: &Value) -> u64 {
    v.get("ts_proc")
        .and_then(|x| x.get("nanos"))
        .and_then(|x| x.as_i64())
        .or_else(|| {
            v.get("ts_recv")
                .and_then(|x| x.get("nanos"))
                .and_then(|x| x.as_i64())
        })
        .map(|n| if n < 0 { 0 } else { n as u64 })
        .unwrap_or(0)
}

fn ensure_instrument(v: &mut Value) {
    if v.get("instrument").is_some() {
        return;
    }

    let ex = v
        .get("exchange")
        .and_then(|x| x.as_str())
        .unwrap_or("Binance")
        .to_string();
    let sym = v
        .get("symbol")
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string();

    // InstrumentKey { exchange: Exchange, symbol: Symbol }
    // Symbol is a newtype => serde expects a STRING for symbol.
    let inst = json!({
        "exchange": ex,
        "symbol": sym
    });

    v.as_object_mut()
        .expect("event must be object")
        .insert("instrument".to_string(), inst);
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        usage();
    }
    let in_path = &args[1];
    let out_path = &args[2];
    let stream = args.get(3).map(|s| s.as_str()).unwrap_or("raw");
    let kind = args.get(4).map(|s| s.as_str()).unwrap_or("event");

    let f = File::open(in_path).with_context(|| format!("open in: {}", in_path))?;
    let r = BufReader::new(f);

    let mut w = EventLogWriter::open_append(out_path, stream, Durability::Buffered)
        .with_context(|| format!("open out: {}", out_path))?;

    let mut n = 0u64;
    for line in r.lines() {
        let line = line.context("read line")?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let mut v: Value = serde_json::from_str(trimmed).context("parse raw json")?;
        ensure_instrument(&mut v);

        let ts_ns = extract_ts_ns(&v);
        let bytes = serde_json::to_vec(&v).context("re-encode json")?;

        w.append_bytes(kind, ts_ns, &bytes).context("append_bytes")?;
        n += 1;
    }

    eprintln!("OK: converted {} events -> {}", n, out_path);
    Ok(())
}
