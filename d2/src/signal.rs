use crate::{Fees, SpreadInput};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GasDecision {
    Gas,
    NoGas,
}

#[derive(Debug, Clone, Copy)]
pub struct Signal {
    pub raw_spread: f64,
    pub net_spread: f64,
    pub net_edge_bps: f64,
    pub decision: GasDecision,
}

#[derive(Debug, Clone, Copy)]
pub struct Thresholds {
    /// absolute net spread threshold (same units as price, e.g. USD)
    pub epsilon: f64,
    /// minimum edge in basis points (bps) relative to buy_price
    pub min_edge_bps: f64,
}

impl Thresholds {
    pub fn pass(&self, net_spread: f64, buy_price: f64) -> bool {
        if buy_price <= 0.0 {
            return false;
        }
        let edge_bps = (net_spread / buy_price) * 10_000.0;
        net_spread > self.epsilon && edge_bps > self.min_edge_bps
    }
}

pub fn compute_signal(
    input: SpreadInput,
    buy_fees: Fees,
    sell_fees: Fees,
    thresholds: Thresholds,
) -> Signal {
    let raw_spread = input.sell_price - input.buy_price;

    let buy_fee = buy_fees.effective(input.buy_is_maker) * input.buy_price;
    let sell_fee = sell_fees.effective(input.sell_is_maker) * input.sell_price;

    let net_spread = raw_spread - buy_fee - sell_fee;
    let net_edge_bps = if input.buy_price > 0.0 {
        (net_spread / input.buy_price) * 10_000.0
    } else {
        f64::NAN
    };

    let decision = if thresholds.pass(net_spread, input.buy_price) {
        GasDecision::Gas
    } else {
        GasDecision::NoGas
    };

    Signal {
        raw_spread,
        net_spread,
        net_edge_bps,
        decision,
    }
}
