use thiserror::Error;

use crate::order::types::OrderState;
use el_core::event::EventType;

#[derive(Debug, Error)]
pub enum OrderFoldError {
    #[error("invalid transition: state={state:?}, event_type={event_type:?}")]
    InvalidTransition { state: OrderState, event_type: EventType },

    #[error("invalid fill accounting: filled_qty={filled_qty} order_qty={order_qty}")]
    Overfill { filled_qty: f64, order_qty: f64 },

    #[error("invalid numeric value in execution payload")]
    InvalidNumber,
}
