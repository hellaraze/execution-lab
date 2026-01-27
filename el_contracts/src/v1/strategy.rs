use super::common::*;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Decision {
    NoGas,
    Gas,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StrategyDecision {
    pub instrument: InstrumentKey,
    pub ts: Timestamp,
    pub decision: Decision,
    pub edge_bps: f64,
}

impl ContractEvent for StrategyDecision {}
