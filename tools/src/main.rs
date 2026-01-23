use anyhow::Result;
use el_core::event::{Event, EventPayload, EventType};
use eventlog::EventLogReader;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let path = args
        .next()
        .unwrap_or_else(|| "/tmp/binance_bbo.eventlog".to_string());
    let n: usize = args
        .next()
        .unwrap_or_else(|| "20".to_string())
        .parse()
        .unwrap_or(20);

    let mut r = EventLogReader::open(&path)?;

    let mut buf: Vec<Event> = Vec::new();
    loop {
        let Some((env, payload)) = r.next()? else {
            break;
        };
        if env.kind != "event" {
            continue;
        }
        let e: Event = serde_json::from_slice(&payload)?;
        buf.push(e);
        if buf.len() > n {
            buf.remove(0);
        }
    }

    for e in buf {
        match (&e.event_type, &e.payload) {
            (EventType::TickerBbo, EventPayload::TickerBbo { bid, ask }) => {
                println!(
                    "TICKER_BBO symbol={} seq={:?} bid={} ask={} ts_ex={:?} ts_proc={}",
                    e.symbol, e.seq, bid, ask, e.ts_exchange, e.ts_proc.nanos
                );
            }
            _ => {
                println!(
                    "EVENT type={:?} symbol={} seq={:?} ts_proc={} payload={:?}",
                    e.event_type, e.symbol, e.seq, e.ts_proc.nanos, e.payload
                );
            }
        }
    }

    Ok(())
}
