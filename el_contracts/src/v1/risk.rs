use super::common::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RiskVerdict {
    Allow,
    Block,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RiskDecision {
    pub instrument: InstrumentKey,
    pub ts: Timestamp,
    pub verdict: RiskVerdict,
    pub reason: &'static str,
}

impl ContractEvent for RiskDecision {}
