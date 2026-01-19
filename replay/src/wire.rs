use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct WireTs {
    pub nanos: u64,
    pub source: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WireEvent {
    pub event_type: String,
    pub exchange: String,
    pub symbol: String,

    pub ts_exchange: Option<WireTs>,
    pub ts_recv: WireTs,
    pub ts_proc: WireTs,

    pub seq: Option<u64>,
    pub schema_version: u16,
    pub integrity_flags: Vec<String>,

    /// In logs it's: {"BookDelta": {...}} or {"BookSnapshot": {...}} etc.
    pub payload: HashMap<String, serde_json::Value>,

    pub meta: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BookLevels {
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TickerBbo {
    pub bid: f64,
    pub ask: f64,
}
