use super::events::OrderEvent;
use super::types::{OrderState, OrderView};
use el_core::event::ExecEvent;

use super::fold_error::OrderFoldError;

fn is_finite(x: f64) -> bool {
    x.is_finite()
}

// NOTE: for now we don't have explicit price/qty in ExecEvent, so filled_qty/avg_px stay 0.
// This still gives us strict transition validation and terminal-state guarantees.
pub fn fold_view(events: &[OrderEvent]) -> Result<OrderView, OrderFoldError> {
    let mut view = OrderView::new();

    for e in events {
        let (state, ev) = (view.state, e.ev);

        view.state = match (state, ev) {
            (OrderState::Created, ExecEvent::OrderPlaced) => OrderState::Sent,
            (OrderState::Sent, ExecEvent::OrderAccepted) => OrderState::Acknowledged,

            (OrderState::Acknowledged, ExecEvent::OrderPartiallyFilled) => OrderState::PartiallyFilled,
            (OrderState::PartiallyFilled, ExecEvent::OrderPartiallyFilled) => OrderState::PartiallyFilled,

            (OrderState::Acknowledged, ExecEvent::OrderFilled) => OrderState::Filled,
            (OrderState::PartiallyFilled, ExecEvent::OrderFilled) => OrderState::Filled,

            (OrderState::Acknowledged, ExecEvent::OrderCanceled) => OrderState::CancelRequested,
            (OrderState::PartiallyFilled, ExecEvent::OrderCanceled) => OrderState::CancelRequested,
            (OrderState::CancelRequested, ExecEvent::OrderCanceled) => OrderState::Cancelled,

            (OrderState::Sent, ExecEvent::OrderRejected)
            | (OrderState::Acknowledged, ExecEvent::OrderRejected)
            | (OrderState::PartiallyFilled, ExecEvent::OrderRejected)
            | (OrderState::CancelRequested, ExecEvent::OrderRejected) => OrderState::Rejected,

            // terminal states: any further events are invalid
            (OrderState::Filled, _)
            | (OrderState::Cancelled, _)
            | (OrderState::Rejected, _)
            | (OrderState::Expired, _) => {
                return Err(OrderFoldError::InvalidTransition { state, event: ev })
            }

            // everything else is invalid
            _ => return Err(OrderFoldError::InvalidTransition { state, event: ev }),
        };

        if !is_finite(view.filled_qty) || !is_finite(view.avg_px) {
            return Err(OrderFoldError::NonFinite);
        }
    }

    Ok(view)
}
