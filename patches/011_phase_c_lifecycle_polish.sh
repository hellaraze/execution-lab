#!/usr/bin/env bash
set -euo pipefail

# =========================================================
# Phase C – Execution lifecycle POLISH (invariants + guards)
# =========================================================

# 1) Add explicit invariants to OrderFsm
cat > exec/src/order_fsm_invariants.rs <<'RS'
use crate::order_state::OrderState::*;
use crate::order_fsm::OrderFsm;
use el_core::event::ExecEvent::*;

impl OrderFsm {
    pub fn assert_invariants(&self) {
        match self.state {
            Filled => {
                assert!(
                    !matches!(self.last_event, Some(OrderCanceled | OrderRejected)),
                    "Filled order cannot be canceled/rejected"
                );
            }
            Canceled | Rejected | Expired => {
                assert!(
                    !matches!(self.last_event, Some(OrderFilled)),
                    "Terminal order cannot be filled"
                );
            }
            _ => {}
        }
    }
}
RS

# 2) Wire invariants into FSM apply()
perl -0777 -i -pe '
s/(self\.state\s*=\s*new_state;\n)/$1        self.assert_invariants();\n/s
' exec/src/order_fsm.rs

# 3) Strengthen ExecGuard (block exec after terminal snapshot)
perl -0777 -i -pe '
s/pub struct ExecGuard \{/pub struct ExecGuard {\n    terminal_seen: bool,\n/s;
s/need_snapshot:\s*false/need_snapshot: false,\n            terminal_seen: false/s;
s/pub fn on_snapshot\(&mut self\)[^{]*\{/pub fn on_snapshot(&mut self) {\n        self.need_snapshot = false;\n        self.terminal_seen = false;\n/s;
s/pub fn on_need_snapshot\(&mut self\)[^{]*\{/pub fn on_need_snapshot(&mut self) {\n        self.need_snapshot = true;\n        self.terminal_seen = true;\n/s;
s/pub fn allow_exec\(&self\)[^{]*\{[^\}]*\}/pub fn allow_exec(&self) -> bool {\n        !self.need_snapshot && !self.terminal_seen\n    }/s
' exec/src/guard.rs

# 4) Crash-safety invariant test
cat > exec/tests/invariant_after_replay.rs <<'RS'
use exec::events::{ExecEvent, OrderId};
use exec::order::snapshot::build_snapshot;
use exec::util::instrument::InstrumentKey;

#[test]
fn invariant_holds_after_replay() {
    let btc = InstrumentKey::new("binance", "BTCUSDT");

    let events = vec![
        ExecEvent::OrderCreated { instrument: btc.clone(), id: OrderId(1) },
        ExecEvent::OrderAcked   { instrument: btc.clone(), id: OrderId(1) },
        ExecEvent::OrderFill {
            instrument: btc.clone(),
            id: OrderId(1),
            filled_qty: 1.0,
            avg_px: 100.0,
        },
    ];

    let (_store, _hash) = build_snapshot(&events).expect("snapshot ok");
}
RS

echo "Phase C lifecycle polish applied"
