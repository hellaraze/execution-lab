use crate::{Fees, Thresholds};

#[derive(Debug, Clone, Copy)]
pub struct Leg {
    pub buy_price: f64,
    pub sell_price: f64,
    pub buy_is_maker: bool,
    pub sell_is_maker: bool,
    pub buy_fees: Fees,
    pub sell_fees: Fees,
}

#[derive(Debug, Clone, Copy)]
pub struct MultiLegSignal {
    pub raw_spread: f64,
    pub net_spread: f64,
    pub net_edge_bps: f64,
    pub pass: bool,
}

pub fn compute_multileg(legs: &[Leg], t: Thresholds) -> MultiLegSignal {
    if legs.is_empty() {
        return MultiLegSignal {
            raw_spread: 0.0,
            net_spread: 0.0,
            net_edge_bps: 0.0,
            pass: false,
        };
    }

    let entry = legs[0].buy_price;
    let mut raw = 0.0;
    let mut net = 0.0;

    for l in legs {
        let leg_raw = l.sell_price - l.buy_price;
        let buy_fee = l.buy_fees.effective(l.buy_is_maker) * l.buy_price;
        let sell_fee = l.sell_fees.effective(l.sell_is_maker) * l.sell_price;

        raw += leg_raw;
        net += leg_raw - buy_fee - sell_fee;
    }

    let edge_bps = (net / entry) * 10_000.0;
    let pass = net > t.epsilon && edge_bps > t.min_edge_bps;

    MultiLegSignal {
        raw_spread: raw,
        net_spread: net,
        net_edge_bps: edge_bps,
        pass,
    }
}
