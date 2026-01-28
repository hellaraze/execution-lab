# PHASE 2 - DEMO / REPLAY RUNTIME (SEALED)

## Goal
Provide a buyer-ready offline demo entrypoint that:
- runs deterministically from an input event log,
- emits stable JSON output,
- produces an evidence JSON bundle for audit.

## Commands

### Demo (offline)
elctl demo --input replay/tests/data/binance_depth_fixture.eventlog --top-n 5 --evidence evidence/demo_evidence.json

### Replay (offline)
elctl replay --input replay/tests/data/binance_depth_fixture.eventlog --top-n 5 --evidence evidence/replay_evidence.json

### Status / Health
elctl status
elctl health
elctl diagnose

## Evidence Bundle
The evidence JSON includes:
- git HEAD
- baseline tag reference
- input path
- executed command
- captured stdout/stderr and exit code

## Definition of Done
- `cargo fmt`, `cargo clippy -D warnings`, `cargo test` all pass
- `elctl demo` succeeds on the fixture and writes evidence JSON
- outputs are ASCII-only and stable JSON
- live mode remains disabled

