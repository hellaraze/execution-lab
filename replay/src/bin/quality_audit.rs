use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};

use el_core::event::Exchange;
use replay::decode::from_wire::decode_event;
use replay::quality::seq::{Gap, SeqTracker};
use replay::wire::WireEvent;

fn parse_exchange(s: &str) -> Exchange {
    match s {
        "Binance" => Exchange::Binance,
        "Okx" => Exchange::Okx,
        "Bybit" => Exchange::Bybit,
        other => Exchange::Other(other.to_string()),
    }
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);

    let mut file = None::<String>;
    let mut exchange = None::<String>;
    let mut symbol = None::<String>;
    let mut max_lines: Option<usize> = None;

    while let Some(a) = args.next() {
        match a.as_str() {
            "--file" => file = args.next(),
            "--exchange" => exchange = args.next(),
            "--symbol" => symbol = args.next(),
            "--max-lines" => {
                max_lines = args
                    .next()
                    .map(|x| x.parse::<usize>().expect("max-lines must be usize"));
            }
            _ => {}
        }
    }

    let file = file.context("missing --file")?;
    let exchange_s = exchange.context("missing --exchange")?;
    let symbol_s = symbol.context("missing --symbol")?;

    let want_ex = parse_exchange(&exchange_s);
    let want_sym = symbol_s;

    let f = File::open(&file).with_context(|| format!("open {}", file))?;
    let r = BufReader::new(f);

    let mut tracker = SeqTracker::new();

    let mut lines = 0usize;
    let mut parsed = 0usize;
    let mut matched = 0usize;
    let mut decoded = 0usize;

    let mut gaps: Vec<Gap> = Vec::new();

    for line in r.lines() {
        let line = line?;
        let t = line.trim();
        if t.is_empty() {
            continue;
        }

        lines += 1;
        if let Some(m) = max_lines {
            if lines > m {
                break;
            }
        }

        let w: WireEvent = match serde_json::from_str(t) {
            Ok(x) => x,
            Err(_) => continue,
        };
        parsed += 1;

        if w.exchange != exchange_s || w.symbol != want_sym {
            continue;
        }
        matched += 1;

        let ev = decode_event(w).context("decode wire->event")?;
        decoded += 1;

        // NOTE: observe() already keys by exchange+symbol internally.
        // We still sanity check:
        if ev.exchange != want_ex {
            continue;
        }

        if let Some(g) = tracker.observe(&ev)? {
            gaps.push(g);
        }
    }

    println!("file={}", file);
    println!("lines_read={}", lines);
    println!("wire_parsed={}", parsed);
    println!("matched_stream={}", matched);
    println!("decoded_events={}", decoded);
    println!("gaps_detected={}", gaps.len());

    if !gaps.is_empty() {
        let mut total_missing: u64 = 0;
        for g in &gaps {
            total_missing += g.to - g.from + 1;
        }
        println!("missing_seq_total={}", total_missing);

        let show = gaps.len().min(10);
        for (i, g) in gaps.iter().take(show).enumerate() {
            println!("gap[{}] {}..{}", i, g.from, g.to);
        }
        if gaps.len() > show {
            println!("(showing first {} gaps)", show);
        }
    }

    Ok(())
}
