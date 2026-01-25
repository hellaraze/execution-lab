#!/usr/bin/env bash
set -euo pipefail

# =========================================================
# FIX:
# - bridge exec::events::ExecEvent -> el_core::event::ExecEvent
# - restore OrderStore::view() for snapshot contract
# =========================================================

# 1) Add conversion impl
cat > exec/src/events/bridge.rs <<'RS'
use el_core::event::ExecEvent as Core;
use crate::events::ExecEvent as Wire;

impl From<Wire> for Core {
    fn from(ev: Wire) -> Self {
        match ev {
            Wire::OrderCreated { instrument, id } =>
                Core::OrderCreated { instrument, id },
            Wire::OrderValidated { instrument, id } =>
                Core::OrderValidated { instrument, id },
            Wire::OrderSent { instrument, id } =>
                Core::OrderSent { instrument, id },
            Wire::OrderAcked { instrument, id } =>
                Core::OrderAcked { instrument, id },
            Wire::OrderFill { instrument, id, filled_qty, avg_px } =>
                Core::OrderFill { instrument, id, filled_qty, avg_px },
            Wire::OrderCancelRequested { instrument, id } =>
                Core::OrderCancelRequested { instrument, id },
            Wire::OrderCancelled { instrument, id } =>
                Core::OrderCancelled { instrument, id },
            Wire::OrderRejected { instrument, id, reason } =>
                Core::OrderRejected { instrument, id, reason },
            Wire::OrderExpired { instrument, id } =>
                Core::OrderExpired { instrument, id },
        }
    }
}
RS

# 2) Re-export bridge
perl -0777 -i -pe 's/pub mod types;/pub mod types;\npub mod bridge;/s' exec/src/events/mod.rs

# 3) Fix OrderStore to use CORE ExecEvent
cat > exec/src/order/store.rs <<'RS'
use std::collections::BTreeMap;

use el_core::event::ExecEvent;
use crate::order_fsm::OrderFsm;
use crate::order_view::OrderView;

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
        let id = ev.order_id().0;
        let fsm = self.orders.entry(id).or_insert_with(OrderFsm::new);
        fsm.apply(ev).map_err(|e| e.to_string())
    }

    pub fn view(&self, id: el_core::event::OrderId) -> Option<OrderView> {
        self.orders.get(&id.0).map(|fsm| OrderView::from(fsm))
    }
}
RS

echo "ExecEvent bridge + OrderStore fixed"
