# Developer / Quality Gate

This repo is engineered for reproducibility and auditability.

## Toolchain (pinned)
Rust toolchain is pinned via `rust-toolchain.toml`.

## One-command quality gate
Run:
```bash
make gate

п
(
set -e

cd ~/projects/execution-lab

mkdir -p docs

cat > docs/STATUS.md <<'EOF'
# EXECUTION-LAB — BASELINE STATUS (SEALED)

This document records **only what is already proven and sealed**.
No plans. No promises. No future statements.
Only artifacts backed by code and tests.

---

## SEALED COMPONENTS

### 1. Contracts (v1)
- Canonical event contracts are frozen
- Explicit versioning is enforced
- Contracts are strictly separated from implementations
- Any change requires a new contract version

Status: **SEALED**

---

### 2. Market Data Integrity (Phase B)
- Canonical `MdEvent` model
- Depth and BBO streams supported
- Sequence and gap detection enforced
- Drift metrics used instead of false mismatch errors
- Deterministic market data replay

Status: **SEALED**

---

### 3. Replay as Absolute Truth
- Replay is the single source of state truth
- Golden hash snapshots enforced
- Deterministic state derivation proven
- Any state is reproducible from the event log

Status: **SEALED**

---

### 4. Execution Lifecycle & Bridge
- Event-sourced order state machine
- Idempotent fills
- Overfill and double-execution guards
- Outbox-only execution bridge
- Exactly-once execution semantics
- Restart-safe and crash-safe invariants validated by tests

Status: **SEALED**

---

### 5. Risk & Observability (Phase G)
- Risk enforced as a mandatory contract
- Explainable and ordered `DecisionReason`
- Structured observability events
- Decision and reason emission
- Fully replay-compatible

Status: **SEALED**

---

### 6. D2 / Pair Scan (Fees-Aware)
- Deterministic GAS / NO_GAS decision model
- Maker and taker fees fully accounted for
- `min_edge_bps` and epsilon thresholds enforced
- A/B pair scan with deterministic golden tests
- NO_GAS decisions are explainable (fees consume spread)

Status: **SEALED**

---

## GLOBAL INVARIANTS

- Event log is the single source of truth
- Replay is deterministic given identical input
- Execution cannot bypass risk controls
- Full post-mortem auditability is guaranteed
- No hidden side effects or implicit state

---

## INTENTIONALLY OUT OF SCOPE

- Graphical user interfaces
- Live trading without hard risk gates
- Strategy optimization or machine learning
- Capital allocation logic

---

## SEAL

This baseline is considered **FROZEN**.
All future work must strictly build on top of this foundation.
Existing guarantees and invariants must not be modified.

