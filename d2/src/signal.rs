use crate::{Fees, SpreadInput};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GasDecision {
    Gas,
    NoGas,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DecisionReason {
    Pass,
    BelowEpsilon,
    BelowMinEdgeBps,
    FeesEatSpread,
    InvalidInput,
}

#[derive(Debug, Clone, Copy)]
pub struct Signal {
    pub raw_spread: f64,
    pub net_spread: f64,
    pub net_edge_bps: f64,
    pub decision: GasDecision,
    pub reason: DecisionReason,
}

#[derive(Debug, Clone, Copy)]
pub struct Thresholds {
    pub epsilon: f64,
    pub min_edge_bps: f64,
}

fn decide(net: f64, edge_bps: f64, t: Thresholds) -> (GasDecision, DecisionReason) {
    if !net.is_finite() || !edge_bps.is_finite() {
        return (GasDecision::NoGas, DecisionReason::InvalidInput);
    }
    if net <= 0.0 {
        return (GasDecision::NoGas, DecisionReason::FeesEatSpread);
    }
    if net <= t.epsilon {
        return (GasDecision::NoGas, DecisionReason::BelowEpsilon);
    }
    if edge_bps <= t.min_edge_bps {
        return (GasDecision::NoGas, DecisionReason::BelowMinEdgeBps);
    }
    (GasDecision::Gas, DecisionReason::Pass)
}

pub fn compute_signal(
    input: SpreadInput,
    buy_fees: Fees,
    sell_fees: Fees,
    t: Thresholds,
) -> Signal {
    let raw_spread = input.sell_price - input.buy_price;

    let buy_fee = buy_fees.effective(input.buy_is_maker) * input.buy_price;
    let sell_fee = sell_fees.effective(input.sell_is_maker) * input.sell_price;

    let net_spread = raw_spread - buy_fee - sell_fee;
    let net_edge_bps = (net_spread / input.buy_price) * 10_000.0;

    let (decision, reason) = decide(net_spread, net_edge_bps, t);

    Signal {
        raw_spread,
        net_spread,
        net_edge_bps,
        decision,
        reason,
    }
}
