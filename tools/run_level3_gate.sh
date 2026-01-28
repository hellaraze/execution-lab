#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")/.."

if command -v cb >/dev/null 2>&1; then
  bash tools/level3_gate.sh 2>&1 | cb
else
  bash tools/level3_gate.sh 2>&1
fi
