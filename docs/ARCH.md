# EXECUTION-LAB - ARCHITECTURE (BASELINE)

This document describes the baseline system architecture and trust boundaries.
It focuses on data flow, determinism, invariants, and auditability.

---

## System Goal

Provide institutional-grade execution infrastructure where:
- the event log is the sole source of truth,
- replay deterministically reconstructs state,
- execution is guarded by risk and is restart-safe,
- every decision and action is traceable post-mortem.

---

## Canonical Data Flow (High Level)

[ Market Data Sources ]
        |
        v
+---------------------+
|  MdAdapters (per-ex)|
|  normalize -> MdEvent|
+---------------------+
        |
        v
+---------------------+
| Event Log (append)  |
| canonical ledger     |
+---------------------+
        |
        v
+---------------------+
| Replay Engine        |
| state = f(eventlog)  |
+---------------------+
        |
        v
+---------------------+        +---------------------+
| Decision Engine      | -----> | Risk Gate (mandatory)|
| (D2 / Strategy)      |        | allow/deny + reason  |
+---------------------+        +---------------------+
        |                              |
        v                              v
+---------------------+        +---------------------+
| Exec Bridge (outbox) | -----> | ExecAdapter (per-ex) |
| exactly-once intent  |        | place/cancel/fills   |
+---------------------+        +---------------------+
        |
        v
+---------------------+
| Audit + Observability|
| immutable evidence   |
+---------------------+

---

## Trust Boundaries

### Boundary A: External Exchanges -> MdAdapters
Assumptions:
- exchange streams may reorder, drop, duplicate, or delay messages
Guarantees:
- adapters emit canonical MdEvent with explicit sequencing / gap signals
- quality issues are measured and never silently ignored

### Boundary B: Event Log
Assumptions:
- append-only storage is the canonical ledger
Guarantees:
- no hidden state outside the event log is required for correctness
- replay can reconstruct state without contacting exchanges

### Boundary C: Replay Engine
Assumptions:
- replay is the only authority on derived state
Guarantees:
- determinism: identical input yields identical output
- golden hash invariants validate reproducibility

### Boundary D: Decision -> Risk -> Execution
Assumptions:
- strategies may be wrong; risk must be right
Guarantees:
- execution cannot bypass risk controls by design
- decisions and denials are explainable via structured reasons

### Boundary E: Exec Bridge / Outbox
Assumptions:
- process can crash at any time
Guarantees:
- exactly-once intent emission
- restart-safe execution progression
- idempotency at the contract level

---

## Core Invariants (Baseline)

### Determinism
- state is derived only from the event log
- replay output is deterministic given identical inputs
- golden hashes detect divergence

### Integrity
- market data gap/sequence issues are detected and surfaced
- execution state machine forbids impossible transitions
- overfill / duplicate execution paths are guarded

### Safety
- risk gate is mandatory for all execution intents
- kill-switch / circuit-breaker semantics exist at the contract boundary

### Auditability
- every decision has an explainable reason
- decision -> intent -> order -> fill chain is reconstructible from logs
- evidence can be exported for post-mortem analysis

---

## What This Baseline Does NOT Assume

- no reliance on "latest" exchange state for correctness
- no reliance on synchronized wall-clock timing for truth
- no reliance on implicit in-memory state surviving restarts

---

## Baseline Seal

This architecture is the frozen reference for Phase 0.
Future phases extend capabilities without breaking the baseline invariants.

