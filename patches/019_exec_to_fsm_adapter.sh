#!/usr/bin/env bash
set -euo pipefail

# =========================================================
# Add exec -> FSM adapter, fix OrderStore/FSM boundary
# =========================================================

# ---- exec/events/adapter.rs ----
cat > exec/src/events/adapter.rs <<'RS'
use crate::events::ExecEvent as ExecEv;
use el_core::event::ExecEvent as FsmEv;

pub fn to_fsm(ev: &ExecEv) -> FsmEv {
    match ev {
        ExecEv::OrderCreated { .. } => FsmEv::OrderPlaced,
        ExecEv::OrderValidated { .. } => FsmEv::OrderAccepted,
        ExecEv::OrderSent { .. } => FsmEv::OrderAccepted,
        ExecEv::OrderAcked { .. } => FsmEv::OrderAccepted,

        ExecEv::OrderFill { filled_qty, .. } => {
            if *filled_qty > 0.0 {
                FsmEv::OrderFilled
            } else {
                FsmEv::OrderPartiallyFilled
            }
        }

        ExecEv::OrderCancelRequested { .. } => FsmEv::OrderCanceled,
        ExecEv::OrderCancelled { .. } => FsmEv::OrderCanceled,
        ExecEv::OrderRejected { .. } => FsmEv::OrderRejected,
        ExecEv::OrderExpired { .. } => FsmEv::OrderRejected,
    }
}
RS

# ---- wire module ----
rg -n "mod events" exec/src/lib.rs >/dev/null || true
perl -0777 -i -pe 's/mod events;/mod events;\npub mod events;\n/s' exec/src/lib.rs

# ---- order/store.rs ----
cat > exec/src/order/store.rs <<'RS'
use crate::events::{ExecEvent, OrderId};
use crate::events::adapter::to_fsm;
use crate::order_fsm::OrderFsm;
use std::collections::HashMap;

#[derive(Default)]
pub struct OrderStore {
    orders: HashMap<u64, OrderFsm>,
}

impl OrderStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_all(&mut self, events: &[ExecEvent]) -> Result<(), String> {
        let mut terminal_seen = false;

        for ev in events {
            if terminal_seen {
                panic!("ExecEvent after terminal state: {:?}", ev);
            }

            self.apply(ev)?;

            if ev.is_terminal() {
                terminal_seen = true;
            }
        }

        Ok(())
    }

    pub fn apply(&mut self, ev: &ExecEvent) -> Result<(), String> {
        let id = ev.order_id().0;
        let fsm = self.orders.entry(id).or_insert_with(OrderFsm::new);
        let fsm_ev = to_fsm(ev);
        fsm.apply(fsm_ev).map_err(|e| e.to_string())
    }

    pub fn snapshot_states(&self) -> Vec<(u64, String)> {
        let mut out: Vec<_> = self
            .orders
            .iter()
            .map(|(id, fsm)| (*id, fsm.state.to_string()))
            .collect();
        out.sort_unstable_by_key(|x| x.0);
        out
    }
}
RS

# ---- order/snapshot.rs ----
cat > exec/src/order/snapshot.rs <<'RS'
use crate::events::ExecEvent;
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
        .map_err(FsmError::Other)?;

    let states = store.snapshot_states();
    let bytes = serde_json::to_vec(&states).expect("serialize snapshot");
    let h = stable_hash_u64(&bytes);

    Ok((store, h))
}
RS

echo "exec → FSM adapter added; OrderStore boundary fixed"
