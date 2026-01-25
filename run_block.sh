#!/usr/bin/env bash
set -euo pipefail

LOG_DIR="${LOG_DIR:-logs}"
mkdir -p "$LOG_DIR"
TS="$(date +%Y%m%d_%H%M%S)"
LOG="$LOG_DIR/run_${TS}.log"

# also keep a stable symlink-like pointer for quick tail
LATEST="$LOG_DIR/_latest.log"
: > "$LOG"
: > "$LATEST"

run() {
  echo "+ $*" | tee -a "$LOG" "$LATEST"
  "$@" 2>&1 | tee -a "$LOG" "$LATEST"
}

fail() {
  echo "=== FAIL (tail 200) ===" | tee -a "$LOG" "$LATEST"
  tail -n 200 "$LOG" | tee -a "$LATEST" >/dev/null
  echo
  echo "LOG: $LOG"
  echo "TAIL: tail -n 200 $LOG"
  exit 1
}

trap fail ERR

run cargo fmt
run cargo test -q
run cargo clippy -q --all-targets --all-features -- -D warnings

echo "OK"
echo "LOG: $LOG"
