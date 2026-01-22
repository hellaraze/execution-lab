use crate::order_state::OrderState;
use el_core::event::ExecEvent;

#[derive(Debug)]
pub struct OrderFsm {
    pub state: OrderState,
}

impl OrderFsm {
    pub fn new() -> Self {
        Self {
            state: OrderState::New,
        }
    }

    pub fn apply(&mut self, ev: ExecEvent) -> Result<(), crate::order_fsm_error::OrderFsmError> {
        use ExecEvent::*;
        use OrderState::*;

        self.state = match (self.state, ev) {
            (New, OrderPlaced) => Placed,
            (Placed, OrderAccepted) => Accepted,
            (Placed, OrderRejected) => Rejected,
            (Accepted, OrderRejected) => Rejected,
            (PartiallyFilled, OrderRejected) => Rejected,

            (Accepted, OrderPartiallyFilled) => PartiallyFilled,
            (PartiallyFilled, OrderPartiallyFilled) => PartiallyFilled,
            (Accepted, OrderFilled) => Filled,
            (PartiallyFilled, OrderFilled) => Filled,
            (Accepted, OrderCanceled) => Canceled,
            (PartiallyFilled, OrderCanceled) => Canceled,
            (s, ev) => {
                return Err(crate::order_fsm_error::OrderFsmError::InvalidTransition {
                    state: s,
                    event: ev,
                })
            }
        };
        Ok(())
    }
}
