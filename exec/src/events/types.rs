use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrderId(pub u64);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ExecEvent {
    OrderCreated { id: OrderId },
    OrderValidated { id: OrderId },
    OrderSent { id: OrderId },
    OrderAcked { id: OrderId },

    // Fill increments filled_qty; avg_px is exchange-reported avg for this fill/aggregate.
    OrderFill { id: OrderId, filled_qty: f64, avg_px: f64 },

    OrderCancelRequested { id: OrderId },
    OrderCancelled { id: OrderId },

    OrderRejected { id: OrderId, reason: String },
    OrderExpired { id: OrderId },
}
