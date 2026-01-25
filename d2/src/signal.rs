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
    pub decision: GasDecision,
}

pub fn compute_signal(input: SpreadInput, buy_fees: Fees, sell_fees: Fees, epsilon: f64) -> Signal {
    let raw_spread = input.sell_price - input.buy_price;

    let buy_fee = buy_fees.effective(input.buy_is_maker) * input.buy_price;
    let sell_fee = sell_fees.effective(input.sell_is_maker) * input.sell_price;

    let net_spread = raw_spread - buy_fee - sell_fee;

    let decision = if net_spread > epsilon {
        GasDecision::Gas
    } else {
        GasDecision::NoGas
    };

    Signal {
        raw_spread,
        net_spread,
        decision,
    }
}
