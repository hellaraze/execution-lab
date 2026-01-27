use super::common::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Bbo {
    pub bid_px: f64,
    pub bid_qty: f64,
    pub ask_px: f64,
    pub ask_qty: f64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MdEvent {
    pub instrument: InstrumentKey,
    pub ts: Timestamp,
    pub bbo: Bbo,
}

impl ContractEvent for MdEvent {}
