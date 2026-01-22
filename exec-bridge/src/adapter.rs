use el_core::event::{Event, EventPayload, EventType};
use exec::events::{ExecEvent, OrderId};

fn oid(s: &str) -> Option<OrderId> {
    let n: u64 = s.parse().ok()?;
    Some(OrderId(n))
}

fn ik(e: &Event) -> exec::util::instrument::InstrumentKey {
    exec::util::instrument::InstrumentKey::new(
        format!("{:?}", e.exchange),
        e.symbol.clone(),
    )
}

pub fn adapt(e: &Event) -> Option<ExecEvent> {
    let instrument = ik(e);

    match (&e.event_type, &e.payload) {
        (EventType::OrderSubmit, EventPayload::OrderSubmit { order_id, .. }) => {
            let id = oid(order_id)?;
            Some(ExecEvent::OrderCreated { instrument, id })
        }

        (EventType::OrderAck, EventPayload::OrderAck { order_id }) => {
            Some(ExecEvent::OrderAcked { instrument, id: oid(order_id)? })
        }

        (EventType::OrderReject, EventPayload::OrderReject { order_id, reason }) => {
            Some(ExecEvent::OrderRejected { instrument, id: oid(order_id)?, reason: reason.clone() })
        }

        (EventType::Fill, EventPayload::Fill { order_id, price, qty, .. }) => {
            Some(ExecEvent::OrderFill { instrument, id: oid(order_id)?, filled_qty: *qty, avg_px: *price })
        }

        (EventType::CancelAck, EventPayload::CancelAck { order_id }) => {
            Some(ExecEvent::OrderCancelled { instrument, id: oid(order_id)? })
        }

        (EventType::CancelRequest, EventPayload::CancelRequest { order_id }) => {
            Some(ExecEvent::OrderCancelRequested { instrument, id: oid(order_id)? })
        }

        _ => None,
    }
}
