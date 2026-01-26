use d2::*;

#[test]
fn reason_and_multileg_work() {
    let t = Thresholds {
        epsilon: 0.0,
        min_edge_bps: 10.0,
    };

    let leg1 = Leg {
        buy_price: 100.0,
        sell_price: 100.6,
        buy_is_maker: true,
        sell_is_maker: true,
        buy_fees: Fees {
            maker: 0.0,
            taker: 0.0,
            rebate: 0.0,
        },
        sell_fees: Fees {
            maker: 0.0,
            taker: 0.0,
            rebate: 0.0,
        },
    };

    let leg2 = Leg {
        buy_price: 100.6,
        sell_price: 101.0,
        buy_is_maker: true,
        sell_is_maker: true,
        buy_fees: Fees {
            maker: 0.0,
            taker: 0.0,
            rebate: 0.0,
        },
        sell_fees: Fees {
            maker: 0.0,
            taker: 0.0,
            rebate: 0.0,
        },
    };

    let m = compute_multileg(&[leg1, leg2], t);
    assert!(m.pass);
    assert!(m.net_edge_bps > 10.0);
}
