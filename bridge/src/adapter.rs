use el_core::event::{Event, EventPayload};
use exec::events::{ExecEvent, OrderId};
use exec::util::instrument::InstrumentKey as ExecInstrumentKey;

fn to_exec_instrument(i: el_core::instrument::InstrumentKey) -> ExecInstrumentKey {
    let sym = i.symbol.0;
    ExecInstrumentKey::new(format!("{:?}", i.exchange), sym)
}

pub fn map_event(e: Event) -> Option<ExecEvent> {
    let instrument = to_exec_instrument(e.instrument);

    match e.payload {
        EventPayload::OrderSubmit { order_id, .. } => {
            let id = OrderId(order_id.parse::<u64>().ok()?);
            Some(ExecEvent::OrderCreated { instrument, id })
        }
        EventPayload::OrderAck { order_id } => {
            let id = OrderId(order_id.parse::<u64>().ok()?);
            Some(ExecEvent::OrderAcked { instrument, id })
        }
        EventPayload::OrderReject { order_id, reason } => {
            let id = OrderId(order_id.parse::<u64>().ok()?);
            Some(ExecEvent::OrderRejected { instrument, id, reason })
        }
        EventPayload::Fill { order_id, price, qty, .. } => {
            let id = OrderId(order_id.parse::<u64>().ok()?);
            Some(ExecEvent::OrderFill { instrument, id, filled_qty: qty, avg_px: price })
        }
        EventPayload::CancelRequest { order_id } => {
            let id = OrderId(order_id.parse::<u64>().ok()?);
            Some(ExecEvent::OrderCancelRequested { instrument, id })
        }
        EventPayload::CancelAck { order_id } => {
            let id = OrderId(order_id.parse::<u64>().ok()?);
            Some(ExecEvent::OrderCancelled { instrument, id })
        }
        _ => None,
    }
}
