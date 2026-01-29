#!/usr/bin/env bash
set -e
cd "$(dirname "${BASH_SOURCE[0]}")/.."

test -f docs/PHASE_15_RELEASE_DRILL_SEALED.md
test -f docs/BUYER_QUICKSTART.md
test -f tools/release/bump_version.sh
test -f tools/release/release_drill.sh

# prior phases must exist
test -f tools/phase11_gate.sh
test -f tools/phase13_gate.sh
test -f tools/phase14_gate.sh

# workflow still present
test -f .github/workflows/release.yml
rg -n 'TAURI_SIGNING_PRIVATE_KEY' .github/workflows/release.yml >/dev/null
rg -n 'mk_latest_json\.sh' .github/workflows/release.yml >/dev/null

echo "PHASE15_GATE_OK"
