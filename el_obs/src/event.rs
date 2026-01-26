use el_core::time::Timestamp;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObsEvent {
    RiskEvaluated {
        ts: Timestamp,
        verdict: String,
    },
}
