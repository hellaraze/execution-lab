use crate::error::ExecError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderState {
    New,
    Open,
    Filled,
    Canceled,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderEvent {
    Accept,
    Fill { fill_id: u64, qty_atoms: u64 },
    Cancel,
    Reject,
}

#[derive(Debug, Clone)]
pub struct OrderData {
    pub state: OrderState,
    pub total_atoms: u64,
    pub filled_atoms: u64,
}

impl OrderData {
    pub fn new(total_atoms: u64) -> Self {
        Self {
            state: OrderState::New,
            total_atoms,
            filled_atoms: 0,
        }
    }
}

pub fn apply(data: &mut OrderData, ev: &OrderEvent) -> Result<(), ExecError> {
    match (&data.state, ev) {
        // Accept only from New
        (OrderState::New, OrderEvent::Accept) => {
            data.state = OrderState::Open;
            Ok(())
        }

        // Fill only from Open
        (OrderState::Open, OrderEvent::Fill { qty_atoms, .. }) => {
            if data.filled_atoms + qty_atoms > data.total_atoms {
                return Err(ExecError::Overfill);
            }

            data.filled_atoms += qty_atoms;

            if data.filled_atoms == data.total_atoms {
                data.state = OrderState::Filled;
            }

            Ok(())
        }

        // No fills after Filled
        (OrderState::Filled, OrderEvent::Fill { .. }) => Err(ExecError::AlreadyFilled),

        // Cancel only from New/Open
        (OrderState::New, OrderEvent::Cancel) | (OrderState::Open, OrderEvent::Cancel) => {
            data.state = OrderState::Canceled;
            Ok(())
        }

        // Reject only from New
        (OrderState::New, OrderEvent::Reject) => {
            data.state = OrderState::Rejected;
            Ok(())
        }

        _ => Err(ExecError::InvalidTransition),
    }
}
