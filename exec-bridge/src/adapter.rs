use el_core::event::{Event, EventPayload, EventType};
use exec::events::{ExecEvent, OrderId};

fn oid(s: &str) -> Option<OrderId> {
    let n: u64 = s.parse().ok()?;
    Some(OrderId(n))
}

pub fn adapt(e: &Event) -> Option<ExecEvent> {
    match (&e.event_type, &e.payload) {
        // core submit/cancel_request -> exec-level lifecycle тоже можно зацепить, но пока пропустим
        (EventType::OrderSubmit, EventPayload::OrderSubmit { .. }) => None,

        (EventType::OrderAck, EventPayload::OrderAck { order_id }) => {
            Some(ExecEvent::OrderAcked { instrument: e.instrument, id: oid(order_id)? })
        }

        (EventType::OrderReject, EventPayload::OrderReject { order_id, reason }) => {
            Some(ExecEvent::OrderRejected { instrument: e.instrument, id: oid(order_id)?, reason: reason.clone() })
        }

        (EventType::Fill, EventPayload::Fill { order_id, price, qty, .. }) => {
            Some(ExecEvent::OrderFill { instrument: e.instrument, id: oid(order_id)?, filled_qty: *qty, avg_px: *price })
        }

        (EventType::CancelAck, EventPayload::CancelAck { order_id }) => {
            Some(ExecEvent::OrderCancelled { instrument: e.instrument, id: oid(order_id)? })
        }

        (EventType::CancelRequest, EventPayload::CancelRequest { .. }) => None,

        _ => None,
    }
}
