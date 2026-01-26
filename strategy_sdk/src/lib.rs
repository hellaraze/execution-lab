use serde::{Serialize, Deserialize};
use el_core::instrument::InstrumentKey;
use d2::{GasDecision, DecisionReason};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyContext {
    pub instrument: InstrumentKey,
}

pub trait Strategy: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn compute(&self, ctx: &StrategyContext, input: &StrategyInput) -> (GasDecision, DecisionReason);
}

pub fn decision_no_gas(reason: DecisionReason) -> (GasDecision, DecisionReason) { (GasDecision::NoGas, reason) }

pub mod example;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyInput {
    // d2::SpreadInput
    pub buy_price: f64,
    pub sell_price: f64,
    pub buy_is_maker: bool,
    pub sell_is_maker: bool,

    // d2::Fees for buy side
    pub buy_maker_bps: f64,
    pub buy_taker_bps: f64,
    pub buy_rebate_bps: f64,

    // d2::Fees for sell side
    pub sell_maker_bps: f64,
    pub sell_taker_bps: f64,
    pub sell_rebate_bps: f64,

    // d2::Thresholds
    pub epsilon: f64,
    pub min_edge_bps: f64,
}
pub mod d2_strategy;
