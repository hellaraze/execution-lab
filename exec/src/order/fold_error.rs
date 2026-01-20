use thiserror::Error;
use crate::order::types::OrderState;
use el_core::event::ExecEvent;

#[derive(Debug, Error)]
pub enum OrderFoldError {
    #[error("invalid transition: state={state:?}, event={event:?}")]
    InvalidTransition { state: OrderState, event: ExecEvent },

    #[error("invalid fill accounting: filled_qty={filled_qty} order_qty={order_qty}")]
    Overfill { filled_qty: f64, order_qty: f64 },

    #[error("non-finite number in fill accounting")]
    NonFinite,
}
