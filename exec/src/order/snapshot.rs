use crate::events::ExecEvent;
use crate::order::{FsmError, OrderStore};
use crate::util::stable_hash_u64;

/// Build deterministic snapshot (OrderStore) from ExecEvent stream,
/// plus stable hash of the entire state for golden/replay testing.
pub fn build_snapshot(events: &[ExecEvent]) -> Result<(OrderStore, u64), FsmError> {
    let mut store = OrderStore::new();
    store.apply_all(events)?;

    // Hash is based on a stable, deterministic representation.
    // We serialize the internal views map as sorted (by OrderId).
    let mut pairs: Vec<(u64, Vec<u8>)> = Vec::new();

    // Access views through public API: re-iterate over known ids is not exposed,
    // so for now we re-serialize by walking events unique ids and querying store.
    // This is deterministic as long as we dedup+sort ids.
    let mut ids: Vec<u64> = events
        .iter()
        .filter_map(|ev| match ev {
            ExecEvent::OrderCreated { id }
            | ExecEvent::OrderValidated { id }
            | ExecEvent::OrderSent { id }
            | ExecEvent::OrderAcked { id }
            | ExecEvent::OrderFill { id, .. }
            | ExecEvent::OrderCancelRequested { id }
            | ExecEvent::OrderCancelled { id }
            | ExecEvent::OrderRejected { id, .. }
            | ExecEvent::OrderExpired { id } => Some(id.0),
        })
        .collect();

    ids.sort_unstable();
    ids.dedup();

    for id_u in ids {
        let id = crate::events::OrderId(id_u);
        let view = store.view(id).expect("id seen in events must exist in store");
        let bytes = serde_json::to_vec(view).expect("serialize OrderView");
        pairs.push((id_u, bytes));
    }

    let all_bytes = serde_json::to_vec(&pairs).expect("serialize snapshot pairs");
    let h = stable_hash_u64(&all_bytes);

    Ok((store, h))
}
