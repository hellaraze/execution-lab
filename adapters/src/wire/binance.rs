use super::{WireEvent, WirePayload};

pub fn map_raw(_raw: &str, seq: u64, ts: u64) -> WireEvent {
    WireEvent {
        source: "binance",
        seq,
        ts_exchange: ts,
        payload: WirePayload::Depth,
    }
}
