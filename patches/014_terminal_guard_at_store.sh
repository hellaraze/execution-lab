#!/usr/bin/env bash
set -euo pipefail

# =========================================================
# FINAL FIX: terminal invariant at OrderStore level
# =========================================================

# 1) Enforce: no ExecEvent allowed after terminal at store-level
perl -0777 -i -pe '
s/pub fn apply_all\(&mut self, events: &\[ExecEvent\]\) -> Result<\(\), OrderError> \{\n/pub fn apply_all(&mut self, events: &[ExecEvent]) -> Result<(), OrderError> {\n        let mut terminal_seen = false;\n/s
' exec/src/order/store.rs

perl -0777 -i -pe '
s/for ev in events \{\n/for ev in events {\n            if terminal_seen {\n                panic!(\"ExecEvent after terminal state: {:?}\", ev);\n            }\n/s
' exec/src/order/store.rs

perl -0777 -i -pe '
s/self\.apply\(ev\)\?;\n/self.apply(ev)?;\n            if ev.is_terminal() {\n                terminal_seen = true;\n            }\n/s
' exec/src/order/store.rs

# 2) Make is_terminal available at events level (idempotent)
if ! rg -n "fn is_terminal" exec/src/events/types.rs >/dev/null 2>&1; then
  perl -0777 -i -pe '
s/impl ExecEvent \{/impl ExecEvent {\n    pub fn is_terminal(&self) -> bool {\n        matches!(self,\n            ExecEvent::OrderCancelled { .. }\n          | ExecEvent::OrderRejected  { .. }\n          | ExecEvent::OrderExpired   { .. }\n        )\n    }\n\n/s
' exec/src/events/types.rs
fi

echo "Terminal invariant enforced at OrderStore level"
