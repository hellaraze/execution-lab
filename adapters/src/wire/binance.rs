use super::{WireEvent, WirePayload};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BookTicker {
    // Binance bookTicker fields:
    // https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md
    #[serde(rename = "b")]
    bid_price: String,
    #[serde(rename = "B")]
    bid_qty: String,
    #[serde(rename = "a")]
    ask_price: String,
    #[serde(rename = "A")]
    ask_qty: String,
}

/// Map a raw Binance bookTicker JSON into a WireEvent::Bbo.
/// Returns None if parse fails.
pub fn map_raw_bbo(raw: &str, seq: u64, ts: u64) -> Option<WireEvent> {
    let t: BookTicker = serde_json::from_str(raw).ok()?;
    let bid_px: f64 = t.bid_price.parse().ok()?;
    let bid_qty: f64 = t.bid_qty.parse().ok()?;
    let ask_px: f64 = t.ask_price.parse().ok()?;
    let ask_qty: f64 = t.ask_qty.parse().ok()?;

    Some(WireEvent {
        source: "binance",
        seq,
        ts_exchange: ts,
        payload: WirePayload::Bbo {
            bid_px,
            bid_qty,
            ask_px,
            ask_qty,
        },
    })
}

// keep old stub for other streams (for now)
pub fn map_raw(_raw: &str, seq: u64, ts: u64) -> WireEvent {
    WireEvent {
        source: "binance",
        seq,
        ts_exchange: ts,
        payload: WirePayload::Depth,
    }
}
