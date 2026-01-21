use anyhow::Result;

use el_core::event::{Event, EventPayload, EventType};

use crate::events::{ExecEvent, OrderId};
use crate::util::instrument::InstrumentKey;

fn hash_order_id(s: &str) -> OrderId {
    let h = blake3::hash(s.as_bytes());
    let mut b = [0u8; 8];
    b.copy_from_slice(&h.as_bytes()[0..8]);
    OrderId(u64::from_le_bytes(b))
}

fn exch_to_string(e: &el_core::event::Exchange) -> String {
    match e {
        el_core::event::Exchange::Binance => "binance".to_string(),
        el_core::event::Exchange::Okx => "okx".to_string(),
        el_core::event::Exchange::Bybit => "bybit".to_string(),
        el_core::event::Exchange::Other(s) => s.clone(),
    }
}

fn instrument_of(ev: &Event) -> InstrumentKey {
    InstrumentKey::new(exch_to_string(&ev.exchange), ev.symbol.clone())
}

pub fn to_exec_event(ev: &Event) -> Result<Option<ExecEvent>> {
    let instrument = instrument_of(ev);

    match (&ev.event_type, &ev.payload) {
        (EventType::OrderSubmit, EventPayload::OrderSubmit { order_id, .. }) => Ok(Some(
            ExecEvent::OrderCreated { instrument, id: hash_order_id(order_id) },
        )),

        (EventType::OrderAck, EventPayload::OrderAck { order_id }) => Ok(Some(
            ExecEvent::OrderAcked { instrument, id: hash_order_id(order_id) },
        )),

        (EventType::OrderReject, EventPayload::OrderReject { order_id, reason }) => Ok(Some(
            ExecEvent::OrderRejected { instrument, id: hash_order_id(order_id), reason: reason.clone() },
        )),

        (EventType::Fill, EventPayload::Fill { order_id, price, qty, .. }) => {
            if !price.is_finite() || !qty.is_finite() || *price <= 0.0 || *qty < 0.0 {
                anyhow::bail!("invalid Fill numbers: price={} qty={}", price, qty);
            }
            Ok(Some(ExecEvent::OrderFill {
                instrument,
                id: hash_order_id(order_id),
                filled_qty: *qty,
                avg_px: *price,
            }))
        }

        (EventType::CancelRequest, EventPayload::CancelRequest { order_id }) => Ok(Some(
            ExecEvent::OrderCancelRequested { instrument, id: hash_order_id(order_id) },
        )),

        (EventType::CancelAck, EventPayload::CancelAck { order_id }) => Ok(Some(
            ExecEvent::OrderCancelled { instrument, id: hash_order_id(order_id) },
        )),

        _ => Ok(None),
    }
}
