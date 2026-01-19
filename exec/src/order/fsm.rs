use crate::events::{ExecEvent, OrderId};
use crate::order::{OrderState, OrderView};

#[derive(Debug, thiserror::Error)]
pub enum FsmError {
    #[error("event order id mismatch: expected {expected:?}, got {got:?}")]
    IdMismatch { expected: OrderId, got: OrderId },

    #[error("illegal transition: state={state:?}, event={event:?}")]
    IllegalTransition { state: OrderState, event: ExecEvent },
}

pub fn apply(view: &mut OrderView, expected_id: OrderId, ev: &ExecEvent) -> Result<(), FsmError> {
    let ev_id = match ev {
        ExecEvent::OrderCreated { id }
        | ExecEvent::OrderValidated { id }
        | ExecEvent::OrderSent { id }
        | ExecEvent::OrderAcked { id }
        | ExecEvent::OrderFill { id, .. }
        | ExecEvent::OrderCancelRequested { id }
        | ExecEvent::OrderCancelled { id }
        | ExecEvent::OrderRejected { id, .. }
        | ExecEvent::OrderExpired { id } => *id,
    };

    if ev_id != expected_id {
        return Err(FsmError::IdMismatch { expected: expected_id, got: ev_id });
    }

    use ExecEvent::*;
    use OrderState::*;

    match (view.state, ev) {
        (Created, OrderCreated { .. }) => Ok(()),
        (Created, OrderValidated { .. }) => {
            view.state = Validated;
            Ok(())
        }

        (Validated, OrderSent { .. }) => {
            view.state = Sent;
            Ok(())
        }
        (Sent, OrderAcked { .. }) => {
            view.state = Acknowledged;
            Ok(())
        }

        (Acknowledged, OrderFill { filled_qty, avg_px, .. }) => {
            view.filled_qty += *filled_qty;
            view.avg_px = *avg_px;
            view.state = PartiallyFilled;
            Ok(())
        }
        (PartiallyFilled, OrderFill { filled_qty, avg_px, .. }) => {
            view.filled_qty += *filled_qty;
            view.avg_px = *avg_px;
            Ok(())
        }

        (Acknowledged, OrderCancelRequested { .. })
        | (PartiallyFilled, OrderCancelRequested { .. }) => {
            view.state = CancelRequested;
            Ok(())
        }

        (CancelRequested, OrderCancelled { .. }) => {
            view.state = Cancelled;
            Ok(())
        }

        // terminal states (accept anytime)
        (_, OrderRejected { .. }) => {
            view.state = Rejected;
            Ok(())
        }
        (_, OrderExpired { .. }) => {
            view.state = Expired;
            Ok(())
        }

        _ => Err(FsmError::IllegalTransition { state: view.state, event: ev.clone() }),
    }
}
