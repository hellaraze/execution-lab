use crate::events::OrderId;
type Price = f64;
type Qty = f64;

#[derive(Debug, Clone)]
pub enum ExecEvent {
    OrderAccepted {
        order_id: OrderId,
    },
    OrderRejected {
        order_id: OrderId,
        reason: String,
    },
    OrderFilled {
        order_id: OrderId,
        qty: Qty,
        price: Price,
    },
    OrderCancelled {
        order_id: OrderId,
    },
}
