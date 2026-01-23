# execution-lab

Execution-grade trading platform built on **event-sourcing**, **deterministic replay** and **provable correctness**.

This is **not** a trading bot.  
This is **infrastructure** — the kind used inside prop desks and quant shops.

---

## Core idea

**Replay is the absolute source of truth.**

Every market event, execution command and execution result is:
- written to an append-only event log
- replayable from genesis
- reproducible bit-by-bit
- auditable offline

If replay disagrees with live — **live is wrong**.

---

## Design principles

### 1. Event-sourcing everywhere
- No hidden mutable state
- No implicit side effects
- State = `fold(events)`

### 2. Deterministic replay
- Same input → same state → same hash
- Golden snapshots used as proofs
- Cross-version determinism is enforced

### 3. Separation of concerns
- Market data ≠ execution
- Adapters ≠ core logic
- Strategy ≠ risk ≠ transport

### 4. Observability by construction
- Gaps are detected, not guessed
- Drift is measured, not panicked on
- Metrics are first-class artifacts

---

## Repository structure


execution-lab/
├── el_contracts/ # Frozen ABI contracts (v1, v2, …)
├── core/ # Core domain types
├── md/ # Market data pipeline
├── exec/ # Execution engine + order FSM
├── connectors/ # Exchange adapters (Binance, etc.)
├── replay/ # Deterministic replay engine
├── tools/ # CLI tools (md_replay, audits)
├── risk/ # Risk engine (contract-driven)
├── strategy/ # Strategy interface / SDK
├── adapters/ # Adapter glue (MD / Exec)
└── app/ # Product shell (future GUI)

---

## Contracts (`el_contracts`)

`el_contracts` defines the **frozen ABI** of the system.

Rules:
- Editing an existing contract is **breaking**
- All evolution happens via versioning (`v1`, `v2`, …)
- Runtime code depends on contracts, never the opposite

### v1 highlights

- `MdEvent` — canonical market data
- `ExCommand` — strategy → execution intent
- `ExEvent` — execution facts
- `Strategy`, `RiskEngine`, `MarketDataAdapter`, `ExecutionAdapter`

### Canonical MD types

Market data is normalized into canonical structures:

- `md::Bbo`
- `md::DepthDiff`

No exchange-specific quirks survive past adapters.

---

## Market Data pipeline


Exchange WS/REST
↓
Wire format (serde)
↓
Adapter normalization
↓
MdEvent (canonical)
↓
Event log
↓
Replay / simulation

### Important rule

**Top-of-book mismatches are not errors.**

Different feeds (depth vs bookTicker) are:
- sampled differently
- aggregated differently
- semantically different

Therefore:
- mismatches → drift metrics
- not fatal assertions

---

## Drift metrics

Drift is tracked as distributions:
- absolute price diff
- tick diff
- percentiles (p50 / p90 / p99 / p999)

These distributions are:
- locked via fixtures
- regression-tested
- version-stable

This turns “market noise” into **quantified behavior**.

---

## Execution engine

Execution is modeled as an **event-sourced FSM**:

- idempotent fills
- overfill guards
- cancel correctness
- exactly-once semantics via outbox

Restart safety is proven by:
- crash simulations
- replay re-hydration
- golden hash checks

---

## Replay

Replay is:
- deterministic
- order-preserving
- hash-verified

Used for:
- backtesting
- audits
- incident analysis
- correctness proofs

If replay cannot reproduce a live result — the live result is invalid.

---

## Tooling

### `md_replay`

CLI tool to:
- replay event logs
- compute drift metrics
- validate invariants
- generate fixtures

Used in CI as a **correctness gate**, not just tests.

---

## Testing philosophy

Not “unit tests everywhere”.

Instead:
- golden tests
- replay invariants
- crash/restart simulations
- contract-level guarantees

Tests answer:
> “Can this system lie?”

---

## Phase status

### Phase A — Contracts freeze ✅
- `el_contracts v1` frozen
- Canonical MD types locked
- Adapters aligned
- Drift fixtures committed

Next phases build **on top**, never by mutation.

---

## Non-goals

- Fast hacks
- Indicator trading
- UI-first design
- Heuristic correctness

---

## Final note

This project is intentionally slow.

Correctness > speed  
Proofs > opinions  
Infrastructure > bots

