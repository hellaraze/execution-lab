use el_core::time::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObsEvent {
    RiskEvaluated { ts: Timestamp, verdict: String },
}
