use crate::events::OrderId;
use el_core::instrument::InstrumentKey;
type Price = f64;
type Qty = f64;

#[derive(Debug, Clone)]
pub struct PlaceOrder {
    pub instrument: InstrumentKey,
    pub order_id: OrderId,
    pub price: Price,
    pub qty: Qty,
    pub side: Side,
}

#[derive(Debug, Clone)]
pub struct CancelOrder {
    pub order_id: OrderId,
}

#[derive(Debug, Clone)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub enum ExecResult {
    Accepted,
    Rejected { reason: String },
}

pub trait ExecAdapter {
    fn place_order(&mut self, cmd: PlaceOrder) -> ExecResult;
    fn cancel_order(&mut self, cmd: CancelOrder) -> ExecResult;
}
