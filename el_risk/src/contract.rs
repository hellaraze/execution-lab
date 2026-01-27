use el_core::instrument::InstrumentKey;
use el_core::time::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskInput {
    pub ts: Timestamp,
    pub instrument: InstrumentKey,
    pub notional: f64,
    pub side: Side,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskVerdict {
    Allow,
    Block(RiskBlockReason),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskBlockReason {
    NotionalLimit,
    DailyLossLimit,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

pub trait RiskEngine {
    fn evaluate(&self, input: &RiskInput) -> RiskVerdict;
}
