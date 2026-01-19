
## Phase 2 — Replay as Source of Truth

Guarantees:
- Deterministic replay
- Deterministic state hash
- Hash chain over state evolution
- Invariant enforcement
- Replay order sensitivity (commutativity ban)

Any change violating these properties MUST break tests.
