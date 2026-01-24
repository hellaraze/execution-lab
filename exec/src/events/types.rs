use serde::{Deserialize, Serialize};

use crate::util::instrument::InstrumentKey;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrderId(pub u64);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ExecEvent {
    OrderCreated {
        instrument: InstrumentKey,
        id: OrderId,
    },
    OrderValidated {
        instrument: InstrumentKey,
        id: OrderId,
    },
    OrderSent {
        instrument: InstrumentKey,
        id: OrderId,
    },
    OrderAcked {
        instrument: InstrumentKey,
        id: OrderId,
    },

    // Fill increments filled_qty; avg_px is exchange-reported avg for this fill/aggregate.
    OrderFill {
        instrument: InstrumentKey,
        id: OrderId,
        filled_qty: f64,
        avg_px: f64,
    },

    OrderCancelRequested {
        instrument: InstrumentKey,
        id: OrderId,
    },
    OrderCancelled {
        instrument: InstrumentKey,
        id: OrderId,
    },

    OrderRejected {
        instrument: InstrumentKey,
        id: OrderId,
        reason: String,
    },
    OrderExpired {
        instrument: InstrumentKey,
        id: OrderId,
    },
}

impl ExecEvent {
    pub fn instrument(&self) -> &InstrumentKey {
        match self {
            ExecEvent::OrderCreated { instrument, .. }
            | ExecEvent::OrderValidated { instrument, .. }
            | ExecEvent::OrderSent { instrument, .. }
            | ExecEvent::OrderAcked { instrument, .. }
            | ExecEvent::OrderFill { instrument, .. }
            | ExecEvent::OrderCancelRequested { instrument, .. }
            | ExecEvent::OrderCancelled { instrument, .. }
            | ExecEvent::OrderRejected { instrument, .. }
            | ExecEvent::OrderExpired { instrument, .. } => instrument,
        }
    }
}
