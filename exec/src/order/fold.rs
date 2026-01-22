use std::collections::HashSet;

use super::events::OrderEvent;
use super::fold_error::OrderFoldError;
use super::types::{OrderState, OrderView};

use el_core::event::{EventPayload, EventType};

fn finite(x: f64) -> bool {
    x.is_finite()
}

fn valid_price_qty(price: f64, qty: f64) -> bool {
    finite(price) && finite(qty) && price > 0.0 && qty >= 0.0
}

pub fn fold_view(events: &[OrderEvent], order_qty: f64) -> Result<OrderView, OrderFoldError> {
    if !finite(order_qty) || order_qty <= 0.0 {
        return Err(OrderFoldError::InvalidNumber);
    }

    let mut view = OrderView::new();
    let mut seen_fill_ids: HashSet<String> = HashSet::new();

    let mut filled_qty: f64 = 0.0;
    let mut notional: f64 = 0.0;

    const EPS: f64 = 1e-12;

    for e in events {
        let state = view.state;

        view.state = match (state, &e.event_type, &e.payload) {
            // submit/ack/reject
            (OrderState::Created, EventType::OrderSubmit, EventPayload::OrderSubmit { .. }) => {
                OrderState::Sent
            }
            (OrderState::Sent, EventType::OrderAck, EventPayload::OrderAck { .. }) => {
                OrderState::Acknowledged
            }

            (OrderState::Sent, EventType::OrderReject, EventPayload::OrderReject { .. })
            | (
                OrderState::Acknowledged,
                EventType::OrderReject,
                EventPayload::OrderReject { .. },
            )
            | (
                OrderState::PartiallyFilled,
                EventType::OrderReject,
                EventPayload::OrderReject { .. },
            )
            | (
                OrderState::CancelRequested,
                EventType::OrderReject,
                EventPayload::OrderReject { .. },
            ) => OrderState::Rejected,

            // fills (idempotent by fill_id)
            (
                OrderState::Acknowledged,
                EventType::Fill,
                EventPayload::Fill {
                    fill_id,
                    price,
                    qty,
                    ..
                },
            )
            | (
                OrderState::PartiallyFilled,
                EventType::Fill,
                EventPayload::Fill {
                    fill_id,
                    price,
                    qty,
                    ..
                },
            ) => {
                if !seen_fill_ids.insert(fill_id.clone()) {
                    // duplicate fill => idempotent noop on accounting/state
                    if (filled_qty - order_qty).abs() <= EPS {
                        OrderState::Filled
                    } else if filled_qty > 0.0 {
                        OrderState::PartiallyFilled
                    } else {
                        OrderState::Acknowledged
                    }
                } else {
                    if !valid_price_qty(*price, *qty) {
                        return Err(OrderFoldError::InvalidNumber);
                    }

                    filled_qty += *qty;
                    notional += *qty * *price;

                    if filled_qty > order_qty + EPS {
                        return Err(OrderFoldError::Overfill {
                            filled_qty,
                            order_qty,
                        });
                    }

                    if (filled_qty - order_qty).abs() <= EPS {
                        OrderState::Filled
                    } else {
                        OrderState::PartiallyFilled
                    }
                }
            }

            // cancel flow
            (
                OrderState::Acknowledged,
                EventType::CancelRequest,
                EventPayload::CancelRequest { .. },
            )
            | (
                OrderState::PartiallyFilled,
                EventType::CancelRequest,
                EventPayload::CancelRequest { .. },
            ) => OrderState::CancelRequested,

            (OrderState::CancelRequested, EventType::CancelAck, EventPayload::CancelAck { .. }) => {
                OrderState::Cancelled
            }

            // terminal states: nothing allowed after
            (OrderState::Filled, _, _)
            | (OrderState::Cancelled, _, _)
            | (OrderState::Rejected, _, _)
            | (OrderState::Expired, _, _) => {
                return Err(OrderFoldError::InvalidTransition {
                    state,
                    event_type: e.event_type.clone(),
                })
            }

            // anything else is invalid
            _ => {
                return Err(OrderFoldError::InvalidTransition {
                    state,
                    event_type: e.event_type.clone(),
                })
            }
        };

        view.filled_qty = filled_qty;
        view.avg_px = if filled_qty > 0.0 {
            notional / filled_qty
        } else {
            0.0
        };

        if !finite(view.filled_qty) || !finite(view.avg_px) {
            return Err(OrderFoldError::InvalidNumber);
        }
    }

    Ok(view)
}
