use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLimits {
    pub max_notional: f64,
}

impl Default for RiskLimits {
    fn default() -> Self {
        Self { max_notional: f64::INFINITY }
    }
}
