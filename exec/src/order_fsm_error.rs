use thiserror::Error;
use crate::order_state::OrderState;
use el_core::event::ExecEvent;

#[derive(Debug, Error)]
pub enum OrderFsmError {
    #[error("invalid transition: state={state:?}, event={event:?}")]
    InvalidTransition {
        state: OrderState,
        event: ExecEvent,
    },
}
