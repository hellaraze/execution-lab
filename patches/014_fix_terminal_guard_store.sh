#!/usr/bin/env bash
set -euo pipefail

# =========================================================
# FIX: terminal invariant at OrderStore level (clean rewrite)
# =========================================================

cat > exec/src/order/store.rs <<'RS'
use crate::events::ExecEvent;
use crate::order::{OrderError, OrderFsm};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct OrderStore {
    orders: BTreeMap<u64, OrderFsm>,
}

impl OrderStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_all(&mut self, events: &[ExecEvent]) -> Result<(), OrderError> {
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

    pub fn apply(&mut self, ev: &ExecEvent) -> Result<(), OrderError> {
        let id = ev.order_id();
        let fsm = self.orders.entry(id).or_insert_with(OrderFsm::new);
        fsm.apply(ev)
    }

    pub fn view(&self, id: crate::events::OrderId) -> Option<&crate::order::OrderView> {
        self.orders.get(&id.0).map(|fsm| &fsm.view)
    }
}
RS

echo "OrderStore rewritten with terminal invariant"
