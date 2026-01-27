use super::common::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ExecEventKind {
    New,
    Fill,
    Cancel,
    Reject,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExecEvent {
    pub instrument: InstrumentKey,
    pub ts: Timestamp,
    pub kind: ExecEventKind,
    pub qty: f64,
    pub px: Option<f64>,
}

impl ContractEvent for ExecEvent {}
