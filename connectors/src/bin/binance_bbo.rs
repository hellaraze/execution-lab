use anyhow::Result;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use url::Url;

use adapters::contracts::BinanceMdMuxAdapterBbo;
use el_contracts::v1::MarketDataAdapter;

use el_core::event::{Event, EventPayload, EventType, Exchange};
use el_core::instrument::InstrumentKey;
use el_core::time::{TimeSource, Timestamp};

use execution_bridge::{Bridge, ExecOutbox};
use time::OffsetDateTime;
use uuid::Uuid;

fn now_nanos() -> i64 {
    OffsetDateTime::now_utc().unix_timestamp_nanos() as i64
}

fn ts(nanos: i64, src: TimeSource) -> Timestamp {
    Timestamp::new(nanos, src)
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let symbol = args.next().unwrap_or_else(|| "BTCUSDT".to_string());
    let log_path = args
        .next()
        .unwrap_or_else(|| "/tmp/binance_bbo.eventlog".to_string());

    // outbox writes core::Event into eventlog with dedup-by-EventId
    let mut outbox = Bridge::open_dedup(
        &log_path,
        "binance_bbo",
        eventlog::writer::Durability::FsyncEvery { n: 1 },
    )?;

    // mux adapter parses raw bookTicker and yields MdEvent::Bbo
    let mut md = BinanceMdMuxAdapterBbo::default();

    let url = Url::parse(&format!(
        "wss://stream.binance.com:9443/ws/{}@bookTicker",
        symbol.to_lowercase()
    ))?;

    eprintln!("connecting: {}", url);
    eprintln!("log: {}", log_path);

    let (ws, _) = connect_async(url).await?;
    let (_, mut read) = ws.split();

    // naive seq: monotonic counter per-connection (v1)
    let mut seq: u64 = 0;

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                eprintln!("CTRL-C: stopping binance_bbo");
                break;
            }
            msg_res = read.next() => {
                let Some(msg_res) = msg_res else { break; };
                let msg = msg_res?;
                if !msg.is_text() { continue; }
                let raw = msg.to_text()?;

                seq += 1;

                // exchange timestamp not guaranteed in WS payload for bookTicker;
                // use receive time as exchange for v1 until we wire E/event_time.
                let _recv_ns = now_nanos();
                let ts_exchange_ms: u64 = 0;

                md.push_raw(raw, seq, ts_exchange_ms);

                for ev in MarketDataAdapter::poll(&mut md) {
                    if let el_contracts::v1::MdEvent::Bbo(b) = ev {
            let bid_px = b.bid_px;
            let ask_px = b.ask_px;
            let ts_ex = b.ts;

                        let now = now_nanos();
                        let sym = symbol.to_uppercase();
                        let core_ev = Event {
                            id: Uuid::new_v4(),
                            event_type: EventType::TickerBbo,
                            exchange: Exchange::Binance,
                            symbol: sym.clone(),
                            instrument: InstrumentKey::new(Exchange::Binance, sym.clone()),
                            ts_exchange: Some(ts_ex),
                            ts_recv: ts(now, TimeSource::Receive),
                            ts_proc: ts(now, TimeSource::Process),
                            seq: Some(seq),
                            schema_version: 1,
                            integrity_flags: vec![],
                            payload: EventPayload::TickerBbo { bid: bid_px, ask: ask_px },
                            meta: std::collections::HashMap::new(),
                        };
                        outbox.publish_once(core_ev)?;
                    }
                }
            }
        }
    }

    Ok(())
}
