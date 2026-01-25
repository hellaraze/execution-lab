use approx::assert_relative_eq;
use d2::*;

#[test]
fn gas_when_net_positive_and_above_thresholds() {
    let input = SpreadInput {
        buy_price: 100.0,
        sell_price: 101.0,
        buy_is_maker: true,
        sell_is_maker: true,
    };

    let buy_fees = Fees {
        maker: 0.0002,
        taker: 0.0007,
        rebate: 0.0001,
    };
    let sell_fees = Fees {
        maker: 0.0002,
        taker: 0.0007,
        rebate: 0.0001,
    };

    let t = Thresholds {
        epsilon: 0.0,
        min_edge_bps: 0.0,
    };

    let s = compute_signal(input, buy_fees, sell_fees, t);

    assert_relative_eq!(s.raw_spread, 1.0);
    assert!(s.net_spread > 0.0);
    assert!(s.net_edge_bps > 0.0);
    assert_eq!(s.decision, GasDecision::Gas);
}

#[test]
fn no_gas_when_thresholds_reject_even_if_positive() {
    let input = SpreadInput {
        buy_price: 100.0,
        sell_price: 100.2,
        buy_is_maker: true,
        sell_is_maker: true,
    };

    let fees = Fees {
        maker: 0.0,
        taker: 0.0,
        rebate: 0.0,
    };

    // raw/net spread = 0.2 => 20 bps
    // require > 25 bps => reject
    let t = Thresholds {
        epsilon: 0.0,
        min_edge_bps: 25.0,
    };

    let s = compute_signal(input, fees, fees, t);

    assert_relative_eq!(s.net_edge_bps, 20.0, epsilon = 1e-9);
    assert_eq!(s.decision, GasDecision::NoGas);
}

#[test]
fn no_gas_when_fees_eat_spread() {
    let input = SpreadInput {
        buy_price: 100.0,
        sell_price: 100.3,
        buy_is_maker: false,
        sell_is_maker: false,
    };

    let fees = Fees {
        maker: 0.0,
        taker: 0.0025,
        rebate: 0.0,
    };

    let t = Thresholds {
        epsilon: 0.0,
        min_edge_bps: 0.0,
    };

    let s = compute_signal(input, fees, fees, t);

    assert!(s.net_spread < 0.0);
    assert_eq!(s.decision, GasDecision::NoGas);
}
