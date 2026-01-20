use anyhow::Result;

use el_core::event::{Event, EventPayload, EventType};

use crate::events::{ExecEvent, OrderId};

fn hash_order_id(s: &str) -> OrderId {
    // stable, deterministic mapping string -> u64
    // NOTE: we deliberately avoid random/crypto; this is for deterministic replay.
    let h = blake3::hash(s.as_bytes());
    let mut b = [0u8; 8];
    b.copy_from_slice(&h.as_bytes()[0..8]);
    OrderId(u64::from_le_bytes(b))
}

pub fn to_exec_event(ev: &Event) -> Result<Option<ExecEvent>> {
    // Only execution-related events are mapped here.
    match (&ev.event_type, &ev.payload) {
        (EventType::OrderSubmit, EventPayload::OrderSubmit { order_id, .. }) => {
            Ok(Some(ExecEvent::OrderCreated { id: hash_order_id(order_id) }))
        }
        (EventType::OrderAck, EventPayload::OrderAck { order_id }) => {
            Ok(Some(ExecEvent::OrderAcked { id: hash_order_id(order_id) }))
        }
        (EventType::OrderReject, EventPayload::OrderReject { order_id, reason }) => {
            Ok(Some(ExecEvent::OrderRejected { id: hash_order_id(order_id), reason: reason.clone() }))
        }
        // Fill in core is incremental qty/price; exec ExecEvent expects cumulative filled_qty + avg_px.
        // For bridge we emit a single "fill delta" as avg_px=price and filled_qty=qty;
        // aggregation is done by OrderStore/apply().
        (EventType::Fill, EventPayload::Fill { order_id, price, qty, .. }) => {
            if !price.is_finite() || !qty.is_finite() || *price <= 0.0 || *qty < 0.0 {
                anyhow::bail!("invalid Fill numbers: price={} qty={}", price, qty);
            }
            Ok(Some(ExecEvent::OrderFill {
                id: hash_order_id(order_id),
                filled_qty: *qty,
                avg_px: *price,
            }))
        }
        (EventType::CancelRequest, EventPayload::CancelRequest { order_id }) => {
            Ok(Some(ExecEvent::OrderCancelRequested { id: hash_order_id(order_id) }))
        }
        (EventType::CancelAck, EventPayload::CancelAck { order_id }) => {
            Ok(Some(ExecEvent::OrderCancelled { id: hash_order_id(order_id) }))
        }

        // non-execution
        _ => Ok(None),
    }
}
