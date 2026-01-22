use execution_bridge::{Bridge, ExecOutbox};
use el_core::event::{Event, EventPayload, EventType, Exchange};
use el_core::time::{Timestamp, TimeSource};
use el_core::instrument::InstrumentKey;
use eventlog::writer::EventLogWriter;
use futures_util::StreamExt;
use orderbook::OrderBook;
use serde::Deserialize;
use std::collections::HashMap;
use time::OffsetDateTime;
use tokio_tungstenite::connect_async;
use url::Url;
use uuid::Uuid;

fn now_nanos() -> i64 {
    OffsetDateTime::now_utc().unix_timestamp_nanos() as i64
}

fn ts(now: i64, src: TimeSource) -> Timestamp {
    Timestamp::new(now, src)
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

#[derive(Debug, Deserialize)]
struct DepthDiff {
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

async fn fetch_snapshot(symbol: &str, limit: u32) -> anyhow::Result<DepthSnapshot> {
    let url = format!(
        "https://api.binance.com/api/v3/depth?symbol={}&limit={}",
        symbol.to_uppercase(),
        limit
    );

    let snap = reqwest::Client::new().get(url).send().await?.json::<DepthSnapshot>().await?;
    Ok(snap)
}

fn emit_snapshot(writer: &mut EventLogWriter, symbol: &str, book: &OrderBook, last_u: u64) -> anyhow::Result<()> {
    let now = now_nanos();
    let bids: Vec<(f64, f64)> = book.bids.iter().map(|(p,q)| (p.0, *q)).collect();
    let asks: Vec<(f64, f64)> = book.asks.iter().map(|(p,q)| (p.0, *q)).collect();

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
        payload: EventPayload::BookSnapshot { bids, asks },
        meta: HashMap::new(),
    };

    writer.write(&ev)?;
    
    Ok(())
}

fn emit_gap(writer: &mut EventLogWriter, symbol: &str, from: u64, to: u64, current_u: u64) -> anyhow::Result<()> {
    let now = now_nanos();
    let ev = Event {
        id: Uuid::new_v4(),
        event_type: EventType::GapDetected,
        exchange: Exchange::Binance,
        symbol: symbol.to_string(),
        instrument: InstrumentKey::new(Exchange::Binance, symbol.to_string()),
        ts_exchange: None,
        ts_recv: ts(now, TimeSource::Receive),
        ts_proc: ts(now, TimeSource::Process),
        seq: Some(current_u),
        schema_version: 1,
        integrity_flags: vec!["depth_gap".to_string()],
        payload: EventPayload::GapDetected { from, to },
        meta: HashMap::new(),
    };
    writer.write(&ev)?;
    
    Ok(())
}

fn emit_resync_started(writer: &mut EventLogWriter, symbol: &str, current_u: u64) -> anyhow::Result<()> {
    let now = now_nanos();
    let ev = Event {
        id: Uuid::new_v4(),
        event_type: EventType::ResyncStarted,
        exchange: Exchange::Binance,
        symbol: symbol.to_string(),
        instrument: InstrumentKey::new(Exchange::Binance, symbol.to_string()),
        ts_exchange: None,
        ts_recv: ts(now, TimeSource::Receive),
        ts_proc: ts(now, TimeSource::Process),
        seq: Some(current_u),
        schema_version: 1,
        integrity_flags: vec!["need_snapshot".to_string()],
        payload: EventPayload::ResyncStarted,
        meta: HashMap::new(),
    };
    writer.write(&ev)?;
    
    Ok(())
}



fn emit_snapshot_outbox(
    outbox: &mut execution_bridge::Bridge,
    symbol: &str,
    book: &OrderBook,
    last_u: u64,
) -> anyhow::Result<()> {
    // reuse existing builder logic by calling existing emit_snapshot into a temp Event,
    // but simplest: call existing emit_snapshot to build Event and then publish_once.
    // We replicate Event creation here to keep it explicit.
    let now = now_nanos();
    let ev = Event {
        id: uuid::Uuid::new_v4(),
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
            bids: book.bids.iter().map(|(p,q)| (p.0, *q)).collect(),
            asks: book.asks.iter().map(|(p,q)| (p.0, *q)).collect(),
        },
        meta: HashMap::new(),
    };
    outbox.publish_once(ev)?;
    Ok(())
}

fn emit_gap_outbox(
    outbox: &mut execution_bridge::Bridge,
    symbol: &str,
    from: u64,
    to: u64,
    current_u: u64,
) -> anyhow::Result<()> {
    let now = now_nanos();
    let ev = Event {
        id: uuid::Uuid::new_v4(),
        event_type: EventType::GapDetected,
        exchange: Exchange::Binance,
        symbol: symbol.to_string(),
        instrument: InstrumentKey::new(Exchange::Binance, symbol.to_string()),
        ts_exchange: None,
        ts_recv: ts(now, TimeSource::Receive),
        ts_proc: ts(now, TimeSource::Process),
        seq: Some(current_u),
        schema_version: 1,
        integrity_flags: vec![],
        payload: EventPayload::GapDetected { from, to },
        meta: HashMap::new(),
    };
    outbox.publish_once(ev)?;
    Ok(())
}

fn emit_resync_started_outbox(
    outbox: &mut execution_bridge::Bridge,
    symbol: &str,
    current_u: u64,
) -> anyhow::Result<()> {
    let now = now_nanos();
    let ev = Event {
        id: uuid::Uuid::new_v4(),
        event_type: EventType::ResyncStarted,
        exchange: Exchange::Binance,
        symbol: symbol.to_string(),
        instrument: InstrumentKey::new(Exchange::Binance, symbol.to_string()),
        ts_exchange: None,
        ts_recv: ts(now, TimeSource::Receive),
        ts_proc: ts(now, TimeSource::Process),
        seq: Some(current_u),
        schema_version: 1,
        integrity_flags: vec!["need_snapshot".to_string()],
        payload: EventPayload::ResyncStarted,
        meta: HashMap::new(),
    };
    outbox.publish_once(ev)?;
    Ok(())
}

pub async fn run_depth_reconstructed(symbol: &str, log_path: &str) -> anyhow::Result<()> {
    // 1) snapshot
    let snap = fetch_snapshot(symbol, 1000).await?;
    let mut book = OrderBook::new();
    let bids = snap.bids.into_iter().map(|x| (x[0].parse().unwrap_or(0.0), x[1].parse().unwrap_or(0.0))).collect::<Vec<_>>();
    let asks = snap.asks.into_iter().map(|x| (x[0].parse().unwrap_or(0.0), x[1].parse().unwrap_or(0.0))).collect::<Vec<_>>();
    book.apply_levels(&bids, &asks);

    let mut last_u = snap.last_update_id;

    let mut outbox = Bridge::open_dedup(log_path, "binance", eventlog::writer::Durability::FsyncEvery { n: 1 })?;
    emit_snapshot_outbox(&mut outbox, &symbol.to_uppercase(), &book, last_u)?;

    // 2) diff stream
    let url = Url::parse(&format!(
        "wss://stream.binance.com:9443/ws/{}@depth@100ms",
        symbol.to_lowercase()
    ))?;

    let (ws, _) = connect_async(url).await?;
    let (_, mut read) = ws.split();

    let mut in_sync = false;
    let mut last_checkpoint_ns: i64 = now_nanos();

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                eprintln!("CTRL-C: stopping binance_depth");
                break;
            }
            msg_res = read.next() => {
                let Some(msg_res) = msg_res else { break; };
                let msg: tokio_tungstenite::tungstenite::Message = msg_res?;

        let msg: tokio_tungstenite::tungstenite::Message = msg;
        if !msg.is_text() { continue; }

        let d: DepthDiff = serde_json::from_str(msg.to_text()?)?;

        // Binance sync rules:
        // First diff after snapshot must satisfy: U <= lastUpdateId+1 <= u
        if !in_sync {
            if d.first_update_id <= last_u + 1 && last_u + 1 <= d.final_update_id {
                in_sync = true;
            } else if d.first_update_id > last_u + 1 {
                // Snapshot too old vs WS. Re-snapshot, emit snapshot, stay out-of-sync.
                let snap = fetch_snapshot(symbol, 1000).await?;
                book = OrderBook::new();
                let bids = snap
                    .bids
                    .into_iter()
                    .map(|x| (x[0].parse().unwrap_or(0.0), x[1].parse().unwrap_or(0.0)))
                    .collect::<Vec<_>>();
                let asks = snap
                    .asks
                    .into_iter()
                    .map(|x| (x[0].parse().unwrap_or(0.0), x[1].parse().unwrap_or(0.0)))
                    .collect::<Vec<_>>();
                book.apply_levels(&bids, &asks);
                last_u = snap.last_update_id;

                emit_snapshot_outbox(&mut outbox, &symbol.to_uppercase(), &book, last_u)?;
                continue;
            } else {
                // still not aligned; skip until aligned
                continue;
            }
        } else {
            // After sync: expect U == last_u + 1
            if d.first_update_id != last_u + 1 {
                // gap/resync
                emit_gap_outbox(&mut outbox, &d.symbol, last_u + 1, d.first_update_id.saturating_sub(1), d.final_update_id)?;
                emit_resync_started_outbox(&mut outbox, &d.symbol, d.final_update_id)?;

                // re-snapshot
                let snap = fetch_snapshot(symbol, 1000).await?;
                book = OrderBook::new();
                let bids = snap.bids.into_iter().map(|x| (x[0].parse().unwrap_or(0.0), x[1].parse().unwrap_or(0.0))).collect::<Vec<_>>();
                let asks = snap.asks.into_iter().map(|x| (x[0].parse().unwrap_or(0.0), x[1].parse().unwrap_or(0.0))).collect::<Vec<_>>();
                book.apply_levels(&bids, &asks);
                last_u = snap.last_update_id;

                emit_snapshot_outbox(&mut outbox, &symbol.to_uppercase(), &book, last_u)?;
                in_sync = false;
                continue;
            }
        }

        // Apply deltas
        let bids = parse_levels(d.bids);
        let asks = parse_levels(d.asks);
        book.apply_levels(&bids, &asks);

        last_u = d.final_update_id;

        // Emit BookDelta event (keeps raw deltas for replay)
        let now = now_nanos();
        let ev = Event {
            id: Uuid::new_v4(),
            event_type: EventType::BookDelta,
            exchange: Exchange::Binance,
            symbol: d.symbol.clone(),
            instrument: InstrumentKey::new(Exchange::Binance, d.symbol.clone()),

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

        // Periodic checkpoint snapshot (every ~5s)
        if now - last_checkpoint_ns >= 5_000_000_000 {
            emit_snapshot_outbox(&mut outbox, &symbol.to_uppercase(), &book, last_u)?;
            last_checkpoint_ns = now;
        }
    
            }
        }
    }

    
    Ok(())
}
