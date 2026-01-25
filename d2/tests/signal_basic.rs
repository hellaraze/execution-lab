use approx::assert_relative_eq;
use d2::*;

#[test]
fn gas_when_net_positive_after_fees() {
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

    let s = compute_signal(input, buy_fees, sell_fees, 0.0);

    assert_relative_eq!(s.raw_spread, 1.0);
    assert!(s.net_spread > 0.0);
    assert_eq!(s.decision, GasDecision::Gas);
}

#[test]
fn no_gas_when_fees_eat_spread() {
    let input = SpreadInput {
        buy_price: 100.0,
        sell_price: 100.3,
        buy_is_maker: false,
        sell_is_maker: false,
    };

    // 0.30 raw spread
    // fees: 0.25% on buy + 0.25% on sell => 0.25 + 0.25075 = 0.50075
    // net = 0.30 - 0.50075 = -0.20075
    let fees = Fees {
        maker: 0.0,
        taker: 0.0025,
        rebate: 0.0,
    };

    let s = compute_signal(input, fees, fees, 0.0);

    assert!(s.net_spread < 0.0);
    assert_eq!(s.decision, GasDecision::NoGas);
}
