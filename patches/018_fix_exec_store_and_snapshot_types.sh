#!/usr/bin/env bash
set -euo pipefail

# =========================================================
# FIX exec OrderStore + snapshot to use exec::events::ExecEvent ONLY
# =========================================================

# ---- order/store.rs ----
cat > exec/src/order/store.rs <<'RS'
use crate::events::{ExecEvent, OrderId};
use crate::order_fsm::OrderFsm;
use std::collections::HashMap;

#[derive(Default)]
pub struct OrderStore {
    orders: HashMap<u64, OrderFsm>,
}

impl OrderStore {
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
        }
    }

    pub fn apply_all(&mut self, events: &[ExecEvent]) -> Result<(), String> {
        let mut terminal_seen = false;

        for ev in events {
            if terminal_seen {
                panic!("ExecEvent after terminal state: {:?}", ev);
            }

            self.apply(ev.clone())?;

            if ev.is_terminal() {
                terminal_seen = true;
            }
        }

        Ok(())
    }

    pub fn apply(&mut self, ev: ExecEvent) -> Result<(), String> {
        let id = ev.order_id().0;
        let fsm = self.orders.entry(id).or_insert_with(OrderFsm::new);
        fsm.apply(ev).map_err(|e| e.to_string())
    }
}
RS

# ---- order/snapshot.rs ----
cat > exec/src/order/snapshot.rs <<'RS'
use crate::events::{ExecEvent, OrderId};
use crate::order::OrderStore;
use crate::util::stable_hash_u64;

#[derive(Debug, thiserror::Error)]
pub enum FsmError {
    #[error("order snapshot error: {0}")]
    Other(String),
}

pub fn build_snapshot(events: &[ExecEvent]) -> Result<(OrderStore, u64), FsmError> {
    let mut store = OrderStore::new();
    store
        .apply_all(events)
        .map_err(|e| FsmError::Other(e))?;

    // stable deterministic hash: order_id -> state
    let mut pairs: Vec<(u64, String)> = Vec::new();

    let mut ids: Vec<u64> = events
        .iter()
        .map(|ev| ev.order_id().0)
        .collect();

    ids.sort_unstable();
    ids.dedup();

    for id in ids {
        let state = store
            .orders
            .get(&id)
            .expect("id must exist")
            .state
            .to_string();
        pairs.push((id, state));
    }

    let bytes = serde_json::to_vec(&pairs).expect("serialize snapshot");
    let h = stable_hash_u64(&bytes);

    Ok((store, h))
}
RS

echo "exec OrderStore + snapshot fixed (exec-only types, no OrderView)"
