use anyhow::Result;
use futures_util::StreamExt;
use orderbook::OrderBook;
use serde::Deserialize;
use std::collections::HashMap;
use time::OffsetDateTime;
use tokio::select;
use tokio_tungstenite::connect_async;
use url::Url;
use uuid::Uuid;

use el_core::event::{Event, EventPayload, EventType, Exchange};
use el_core::instrument::InstrumentKey;
use el_core::time::{TimeSource, Timestamp};

use execution_bridge::{Bridge, ExecOutbox};

#[derive(Debug, Deserialize)]
struct WsDepthDiff {
    #[serde(rename = "e")]
    _event_type: String,
    #[serde(rename = "E")]
    event_time_ms: u64,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "U")]
    first_update_id: u64,
    #[serde(rename = "u")]
    final_update_id: u64,
    #[serde(rename = "b")]
    bids: Vec<[String; 2]>,
    #[serde(rename = "a")]
    asks: Vec<[String; 2]>,
}

#[derive(Debug, Deserialize)]
struct WsBookTicker {
    #[serde(rename = "u")]
    _update_id: u64,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "b")]
    bid_price: String,
    #[serde(rename = "B")]
    _bid_qty: String,
    #[serde(rename = "a")]
    ask_price: String,
    #[serde(rename = "A")]
    _ask_qty: String,
    // Some bookTicker payloads include E; if absent, your code may set None anyway.
    #[serde(rename = "E")]
    event_time_ms: Option<u64>,
}
fn now_nanos() -> i64 {
    OffsetDateTime::now_utc().unix_timestamp_nanos() as i64
}
fn ts(nanos: i64, src: TimeSource) -> Timestamp {
    Timestamp::new(nanos, src)
}
fn parse_levels(levels: Vec<[String; 2]>) -> Vec<(f64, f64)> {
    levels
        .into_iter()
        .map(|x| (x[0].parse().unwrap_or(0.0), x[1].parse().unwrap_or(0.0)))
        .collect()
}

#[derive(Debug, Deserialize)]
struct DepthSnapshot {
    #[serde(rename = "lastUpdateId")]
    last_update_id: u64,
    bids: Vec<[String; 2]>,
    asks: Vec<[String; 2]>,
}

async fn fetch_snapshot(symbol: &str, limit: u32) -> anyhow::Result<DepthSnapshot> {
    let url = format!(
        "https://api.binance.com/api/v3/depth?symbol={}&limit={}",
        symbol.to_uppercase(),
        limit
    );

    let snap = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json::<DepthSnapshot>()
        .await?;
    Ok(snap)
}

fn emit_snapshot_outbox(
    outbox: &mut Bridge,
    symbol: &str,
    book: &OrderBook,
    last_u: u64,
) -> anyhow::Result<()> {
    let now = now_nanos();
    let ev = Event {
        id: Uuid::new_v4(),
        event_type: EventType::BookSnapshot,
        exchange: Exchange::Binance,
        symbol: symbol.to_string(),
        instrument: InstrumentKey::new(Exchange::Binance, symbol.to_string()),
        ts_exchange: None,
        ts_recv: ts(now, TimeSource::Receive),
        ts_proc: ts(now, TimeSource::Process),
        seq: Some(last_u),
        schema_version: 1,
        integrity_flags: vec![],
        payload: EventPayload::BookSnapshot {
            bids: book.bids.iter().map(|(p, q)| (p.0, *q)).collect(),
            asks: book.asks.iter().map(|(p, q)| (p.0, *q)).collect(),
        },
        meta: HashMap::new(),
    };
    outbox.publish_once(ev)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let symbol = args.next().unwrap_or_else(|| "BTCUSDT".to_string());
    let log_path = args
        .next()
        .unwrap_or_else(|| "/tmp/binance_md.eventlog".to_string());

    eprintln!("symbol={} log={}", symbol, log_path);

    let mut outbox = Bridge::open_dedup(
        &log_path,
        "binance-md",
        eventlog::writer::Durability::FsyncEvery { n: 1 },
    )?;

    // 1) snapshot init
    let snap = fetch_snapshot(&symbol, 1000).await?;
    let mut book = OrderBook::new();
    let bids = parse_levels(snap.bids);
    let asks = parse_levels(snap.asks);
    book.apply_levels(&bids, &asks);

    let mut last_u = snap.last_update_id;
    emit_snapshot_outbox(&mut outbox, &symbol.to_uppercase(), &book, last_u)?;

    // 2) depth stream + bbo stream
    let url_depth = Url::parse(&format!(
        "wss://stream.binance.com:9443/ws/{}@depth@100ms",
        symbol.to_lowercase()
    ))?;
    let url_bbo = Url::parse(&format!(
        "wss://stream.binance.com:9443/ws/{}@bookTicker",
        symbol.to_lowercase()
    ))?;

    let (ws_depth, _) = connect_async(url_depth).await?;
    let (ws_bbo, _) = connect_async(url_bbo).await?;
    let (_, mut rd_depth) = ws_depth.split();
    let (_, mut rd_bbo) = ws_bbo.split();

    // depth sync flag
    let mut in_sync = false;

    loop {
        select! {
            _ = tokio::signal::ctrl_c() => {
                eprintln!("CTRL-C: stopping binance_md");
                break;
            }

            msg_res = rd_depth.next() => {
                let Some(msg_res) = msg_res else { break; };
                let msg = msg_res?;
                if !msg.is_text() { continue; }

                let d: WsDepthDiff = serde_json::from_str(msg.to_text()?)?;

                if !in_sync {
                    // First diff after snapshot must satisfy: U <= lastUpdateId+1 <= u
                    if d.first_update_id <= last_u + 1 && last_u + 1 <= d.final_update_id {
                        in_sync = true;
                    } else if d.first_update_id > last_u + 1 {
                        // snapshot too old, re-snapshot
                        let snap = fetch_snapshot(&symbol, 1000).await?;
                        book = OrderBook::new();
                        let bids = parse_levels(snap.bids);
                        let asks = parse_levels(snap.asks);
                        book.apply_levels(&bids, &asks);
                        last_u = snap.last_update_id;
                        emit_snapshot_outbox(&mut outbox, &symbol.to_uppercase(), &book, last_u)?;
                        continue;
                    } else {
                        continue;
                    }
                } else {
                    // After sync: expect U == last_u + 1, otherwise resync
                    if d.first_update_id != last_u + 1 {
                        // minimal: re-snapshot (we skip GapDetected/Resync events here; can add later)
                        let snap = fetch_snapshot(&symbol, 1000).await?;
                        book = OrderBook::new();
                        let bids = parse_levels(snap.bids);
                        let asks = parse_levels(snap.asks);
                        book.apply_levels(&bids, &asks);
                        last_u = snap.last_update_id;
                        emit_snapshot_outbox(&mut outbox, &symbol.to_uppercase(), &book, last_u)?;
                        in_sync = false;
                        continue;
                    }
                }

                // apply deltas + emit BookDelta
                let bids = parse_levels(d.bids);
                let asks = parse_levels(d.asks);
                book.apply_levels(&bids, &asks);
                last_u = d.final_update_id;

                let now = now_nanos();
                let ev = Event {
                    id: Uuid::new_v4(),
                    event_type: EventType::BookDelta,
                    exchange: Exchange::Binance,
                    symbol: symbol.to_uppercase(),
                    instrument: InstrumentKey::new(Exchange::Binance, symbol.to_uppercase()),
                    ts_exchange: Some(ts((d.event_time_ms as i64) * 1_000_000, TimeSource::Exchange)),
                    ts_recv: ts(now, TimeSource::Receive),
                    ts_proc: ts(now, TimeSource::Process),
                    seq: Some(d.final_update_id),
                    schema_version: 1,
                    integrity_flags: vec![],
                    payload: EventPayload::BookDelta { bids, asks },
                    meta: HashMap::new(),
                };
                outbox.publish_once(ev)?;
            }

            msg_res = rd_bbo.next() => {
                let Some(msg_res) = msg_res else { break; };
                let msg = msg_res?;
                if !msg.is_text() { continue; }

                let t: WsBookTicker = serde_json::from_str(msg.to_text()?)?;
                let bid: f64 = t.bid_price.parse().unwrap_or(0.0);
                let ask: f64 = t.ask_price.parse().unwrap_or(0.0);

                let now = now_nanos();
                let ts_ex = t.event_time_ms
                    .map(|ms| ts((ms as i64) * 1_000_000, TimeSource::Exchange));

                let ev = Event {
                    id: Uuid::new_v4(),
                    event_type: EventType::TickerBbo,
                    exchange: Exchange::Binance,
                    symbol: symbol.to_uppercase(),
                    instrument: InstrumentKey::new(Exchange::Binance, symbol.to_uppercase()),
                    ts_exchange: ts_ex,
                    ts_recv: ts(now, TimeSource::Receive),
                    ts_proc: ts(now, TimeSource::Process),
                    seq: None, // bookTicker has no exchange seq
                    schema_version: 1,
                    integrity_flags: vec![],
                    payload: EventPayload::TickerBbo { bid, ask },
                    meta: HashMap::new(),
                };
                outbox.publish_once(ev)?;
            }
        }
    }

    Ok(())
}
