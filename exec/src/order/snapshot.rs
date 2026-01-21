use crate::events::ExecEvent;
use crate::order::OrderStore;
use crate::util::stable_hash_u64;


#[derive(Debug, thiserror::Error)]
pub enum FsmError {
    #[error("order snapshot error: {0}")]
    Other(String),
}

/// Build deterministic snapshot (OrderStore) from ExecEvent stream,
/// plus stable hash of the entire state for golden/replay testing.
pub fn build_snapshot(events: &[ExecEvent]) -> Result<(OrderStore, u64), FsmError> {
    let mut store = OrderStore::new();
    store.apply_all(events).map_err(|e| FsmError::Other(e.to_string()))?;

    // Hash is based on a stable, deterministic representation.
    // We serialize the internal views map as sorted (by OrderId).
    let mut pairs: Vec<(u64, Vec<u8>)> = Vec::new();

    // Access views through public API: re-iterate over known ids is not exposed,
    // so for now we re-serialize by walking events unique ids and querying store.
    // This is deterministic as long as we dedup+sort ids.
    let mut ids: Vec<u64> = events
        .iter()
        .filter_map(|ev| match ev {
            ExecEvent::OrderCreated { id, .. }
            | ExecEvent::OrderValidated { id, .. }
            | ExecEvent::OrderSent { id, .. }
            | ExecEvent::OrderAcked { id, .. }
            | ExecEvent::OrderFill { id, .. }
            | ExecEvent::OrderCancelRequested { id, .. }
            | ExecEvent::OrderCancelled { id, .. }
            | ExecEvent::OrderRejected { id, .. }
            | ExecEvent::OrderExpired { id, .. } => Some(id.0),
        })
        .collect();

    ids.sort_unstable();
    ids.dedup();

    for id_u in ids {
        let id = crate::events::OrderId(id_u);
        let view = store.view(id).expect("id seen in events must exist in store");
        let bytes = serde_json::to_vec(&view).expect("serialize OrderView");
        pairs.push((id_u, bytes));
    }

    let all_bytes = serde_json::to_vec(&pairs).expect("serialize snapshot pairs");
    let h = stable_hash_u64(&all_bytes);

    Ok((store, h))
}

use std::collections::BTreeMap;
use crate::util::instrument::InstrumentKey;

/// Build deterministic multi-instrument snapshot hash.
/// Returns per-instrument OrderStore map + global hash.
pub fn build_snapshot_multi(events: &[ExecEvent]) -> Result<(BTreeMap<InstrumentKey, OrderStore>, u64), FsmError> {
    // partition stores by instrument
    let mut stores: BTreeMap<InstrumentKey, OrderStore> = BTreeMap::new();
    for ev in events {
        let key = ev.instrument().clone();
        let store = stores.entry(key).or_insert_with(OrderStore::new);
        store.apply(ev).map_err(|e| FsmError::Other(e.to_string()))?;
    }

    // stable hash: instrument -> sorted [(order_id, view_bytes)]
    let mut out: Vec<(InstrumentKey, Vec<(u64, Vec<u8>)>)> = Vec::new();

    for (key, store) in &stores {
        // collect ids from events for this instrument
        let mut ids: Vec<u64> = events.iter().filter_map(|ev| {
            if ev.instrument() != key { return None; }
            match ev {
                ExecEvent::OrderCreated { id, .. }
                | ExecEvent::OrderValidated { id, .. }
                | ExecEvent::OrderSent { id, .. }
                | ExecEvent::OrderAcked { id, .. }
                | ExecEvent::OrderFill { id, .. }
                | ExecEvent::OrderCancelRequested { id, .. }
                | ExecEvent::OrderCancelled { id, .. }
                | ExecEvent::OrderRejected { id, .. }
                | ExecEvent::OrderExpired { id, .. } => Some(id.0),
            }
        }).collect();

        ids.sort_unstable();
        ids.dedup();

        let mut pairs: Vec<(u64, Vec<u8>)> = Vec::new();
        for id_u in ids {
            let id = crate::events::OrderId(id_u);
            let view = store.view(id).expect("id seen in events must exist in store");
            let bytes = serde_json::to_vec(&view).expect("serialize OrderView");
            pairs.push((id_u, bytes));
        }

        out.push((key.clone(), pairs));
    }

    // BTreeMap iteration already sorted by InstrumentKey, but we hash Vec to be explicit.
    let all_bytes = serde_json::to_vec(&out).expect("serialize multi snapshot");
    let h = stable_hash_u64(&all_bytes);

    Ok((stores, h))
}
