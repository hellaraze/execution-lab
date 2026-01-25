#!/usr/bin/env bash
set -euo pipefail

# =========================================================
# FIX: terminal invariant in OrderStore (project-accurate)
# =========================================================

cat > exec/src/order/store.rs <<'RS'
use crate::events::ExecEvent;
use crate::order_fsm::OrderFsm;
use crate::order::OrderView;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct OrderStore {
    orders: BTreeMap<u64, OrderFsm>,
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
        let id = match ev {
            ExecEvent::OrderCreated { id, .. }
            | ExecEvent::OrderValidated { id, .. }
            | ExecEvent::OrderSent { id, .. }
            | ExecEvent::OrderAcked { id, .. }
            | ExecEvent::OrderFill { id, .. }
            | ExecEvent::OrderCancelRequested { id, .. }
            | ExecEvent::OrderCancelled { id, .. }
            | ExecEvent::OrderRejected { id, .. }
            | ExecEvent::OrderExpired { id, .. } => id.0,
        };

        let fsm = self.orders.entry(id).or_insert_with(OrderFsm::new);
        fsm.apply(ev).map_err(|e| e.to_string())
    }

    pub fn view(&self, id: crate::events::OrderId) -> Option<&OrderView> {
        self.orders.get(&id.0).map(|fsm| &fsm.view)
    }
}
RS

# ---- snapshot.rs: fix E0282 ----
perl -0777 -i -pe '
s/map_err\\(\\|e\\| FsmError::Other\\(e.to_string\\(\\)\\)\\)/map_err(|e: crate::order::OrderError| FsmError::Other(e.to_string()))/g
' exec/src/order/snapshot.rs

echo "Terminal invariant fixed correctly"
