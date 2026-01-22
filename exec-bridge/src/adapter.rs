use el_core::event::{Event, EventPayload, EventType};
use exec::events::{ExecEvent, OrderId};

fn oid(s: &str) -> Option<OrderId> {
    let n: u64 = s.parse().ok()?;
    Some(OrderId(n))
}

fn ik(e: &Event) -> exec::util::instrument::InstrumentKey {
    // best-effort: map by exchange + symbol string
    // core: exchange enum + symbol string already present on Event
    let ex = match &e.exchange {
        el_core::event::Exchange::Binance => exec::util::instrument::Exchange::Binance,
        el_core::event::Exchange::Okx => exec::util::instrument::Exchange::Okx,
        el_core::event::Exchange::Bybit => exec::util::instrument::Exchange::Bybit,
        el_core::event::Exchange::Other(s) => exec::util::instrument::Exchange::Other(s.clone()),
    };
    exec::util::instrument::InstrumentKey::new(ex, &e.symbol)
}

pub fn adapt(e: &Event) -> Option<ExecEvent> {
    let instrument = ik(e);

    match (&e.event_type, &e.payload) {
        (EventType::OrderSubmit, EventPayload::OrderSubmit { .. }) => None,

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

        (EventType::CancelRequest, EventPayload::CancelRequest { .. }) => None,

        _ => None,
    }
}
