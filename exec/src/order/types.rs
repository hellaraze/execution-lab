use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderState {
    Created,
    Validated,
    Sent,
    Acknowledged,
    PartiallyFilled,
    Filled,
    CancelRequested,
    Cancelled,
    Rejected,
    Expired,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderView {
    pub state: OrderState,
    pub filled_qty: f64,
    pub avg_px: f64,
}

impl OrderView {
    pub fn new() -> Self {
        Self {
            state: OrderState::Created,
            filled_qty: 0.0,
            avg_px: 0.0,
        }
    }
}

impl Default for OrderView {
    fn default() -> Self {
        Self::new()
    }
}
