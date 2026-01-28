#!/usr/bin/env bash
set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

TS="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
OUT="repro_baseline_${TS}.log"

{
  echo "=== EXECUTION-LAB BASELINE REPRO ==="
  echo "ts_utc: ${TS}"
  echo

  echo "=== PLATFORM ==="
  uname -a || true
  echo

  echo "=== TOOLCHAIN ==="
  if [ -f rust-toolchain.toml ]; then
    echo "--- rust-toolchain.toml ---"
    cat rust-toolchain.toml
    echo "---------------------------"
  fi
  rustc -V
  cargo -V
  echo

  echo "=== GIT ==="
  git rev-parse --is-inside-work-tree >/dev/null 2>&1 && {
    git rev-parse HEAD
    git status --porcelain
  } || true
  echo

  echo "=== QUALITY GATE ==="
  if [ -x tools/quality_gate.sh ]; then
    tools/quality_gate.sh
  else
    echo "WARN: tools/quality_gate.sh not found or not executable"
  fi
  echo

  echo "=== DETERMINISTIC DEMO (D2 replay-ro scan) ==="
  # Adjust paths only if these fixtures differ in your repo.
  FIXTURE="replay/tests/data/binance_depth_fixture.eventlog"
  if [ -f "$FIXTURE" ]; then
    cargo run -q -p d2 --features replay-ro --bin d2_scan -- "$FIXTURE" --top-n 20
  else
    echo "WARN: fixture not found: $FIXTURE"
  fi
  echo

  echo "=== DETERMINISTIC DEMO (D2 pair scan golden test) ==="
  # This should be a deterministic test that proves reproducibility.
  cargo test -q -p d2 pair_scan_golden_gas

  echo
  echo "=== REPRO DONE ==="
} >"$OUT" 2>&1

echo "WROTE: $OUT"
