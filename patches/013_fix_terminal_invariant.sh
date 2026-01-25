#!/usr/bin/env bash
set -euo pipefail

# =========================================================
# FIX: terminal invariant MUST live in OrderFsm::apply
# =========================================================

# 1) Усиливаем FSM: запрет apply после terminal
perl -0777 -i -pe '
s/pub fn apply\(&mut self, event: &ExecEvent\) -> Result<\(\), OrderError> \{\n/pub fn apply(&mut self, event: &ExecEvent) -> Result<(), OrderError> {\n        if self.state.is_terminal() {\n            panic!(\"event after terminal state: {:?}\", event);\n        }\n/s
' exec/src/order_fsm.rs

# 2) Явный helper для terminal state
perl -0777 -i -pe '
s/impl OrderState \{/impl OrderState {\n    pub fn is_terminal(&self) -> bool {\n        matches!(self, Self::Cancelled | Self::Rejected | Self::Expired | Self::Filled)\n    }\n/s
' exec/src/order_state.rs

echo "Terminal invariant enforced in OrderFsm"
