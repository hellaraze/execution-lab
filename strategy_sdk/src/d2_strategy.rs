use crate::{Strategy, StrategyContext, StrategyInput};
use d2::{compute_signal, DecisionReason, Fees, GasDecision, SpreadInput, Thresholds};

pub struct D2SignalStrategy;

impl Strategy for D2SignalStrategy {
    fn name(&self) -> &'static str {
        "d2_signal"
    }

    fn compute(
        &self,
        _ctx: &StrategyContext,
        input: &StrategyInput,
    ) -> (GasDecision, DecisionReason) {
        let si = SpreadInput {
            buy_price: input.buy_price,
            sell_price: input.sell_price,
            buy_is_maker: input.buy_is_maker,
            sell_is_maker: input.sell_is_maker,
        };

        let buy_fees = Fees {
            maker: input.buy_maker_bps,
            taker: input.buy_taker_bps,
            rebate: input.buy_rebate_bps,
        };
        let sell_fees = Fees {
            maker: input.sell_maker_bps,
            taker: input.sell_taker_bps,
            rebate: input.sell_rebate_bps,
        };
        let t = Thresholds {
            epsilon: input.epsilon,
            min_edge_bps: input.min_edge_bps,
        };

        let sig = compute_signal(si, buy_fees, sell_fees, t);
        (sig.decision, sig.reason)
    }
}
