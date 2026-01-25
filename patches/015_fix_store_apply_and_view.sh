#!/usr/bin/env bash
set -euo pipefail

# =========================================================
# FIX OrderStore:
# - pass ExecEvent by value
# - remove invalid view() method
# =========================================================

cat > exec/src/order/store.rs <<'RS'
use crate::events::ExecEvent;
use crate::order_fsm::OrderFsm;
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

            self.apply(ev.clone())?;

            if ev.is_terminal() {
                terminal_seen = true;
            }
        }

        Ok(())
    }

    pub fn apply(&mut self, ev: ExecEvent) -> Result<(), String> {
        let id = match &ev {
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
}
RS

echo "OrderStore fixed: apply-by-value, view removed"
