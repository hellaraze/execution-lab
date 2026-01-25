#!/usr/bin/env bash
set -euo pipefail

cat >> core/src/event.rs <<'RS'

impl ExecEvent {
    pub fn order_id(&self) -> OrderId {
        match self {
            ExecEvent::OrderCreated { id, .. }
            | ExecEvent::OrderValidated { id, .. }
            | ExecEvent::OrderSent { id, .. }
            | ExecEvent::OrderAcked { id, .. }
            | ExecEvent::OrderFill { id, .. }
            | ExecEvent::OrderCancelRequested { id, .. }
            | ExecEvent::OrderCancelled { id, .. }
            | ExecEvent::OrderRejected { id, .. }
            | ExecEvent::OrderExpired { id, .. } => *id,
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            ExecEvent::OrderCancelled { .. }
                | ExecEvent::OrderRejected { .. }
                | ExecEvent::OrderExpired { .. }
                | ExecEvent::OrderFill { .. } // filled == terminal in FSM
        )
    }
}
RS

echo "core ExecEvent helpers added"
