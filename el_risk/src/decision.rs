use super::contract::*;
use super::limits::RiskLimits;

pub struct SimpleRisk {
    pub limits: RiskLimits,
}

impl RiskEngine for SimpleRisk {
    fn evaluate(&self, input: &RiskInput) -> RiskVerdict {
        if input.notional > self.limits.max_notional {
            return RiskVerdict::Block(RiskBlockReason::NotionalLimit);
        }
        RiskVerdict::Allow
    }
}
