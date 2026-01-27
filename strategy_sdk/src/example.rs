use crate::{Strategy, StrategyContext};
use d2::{DecisionReason, GasDecision};

pub struct AlwaysNoGas;

impl Strategy for AlwaysNoGas {
    fn name(&self) -> &'static str {
        "always_no_gas"
    }

    fn compute(
        &self,
        _ctx: &StrategyContext,
        _input: &crate::StrategyInput,
    ) -> (GasDecision, DecisionReason) {
        (GasDecision::NoGas, DecisionReason::BelowMinEdgeBps)
    }
}
