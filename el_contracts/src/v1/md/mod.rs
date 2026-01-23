use el_core::instrument::InstrumentKey;
use el_core::time::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bbo {
    pub instrument: InstrumentKey,
    pub ts: Timestamp,
    pub bid_px: f64,
    pub bid_qty: f64,
    pub ask_px: f64,
    pub ask_qty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepthDiff {
    pub instrument: InstrumentKey,
    pub ts: Timestamp,
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
}
