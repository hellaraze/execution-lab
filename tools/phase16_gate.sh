#!/usr/bin/env bash
set -e
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

test -f docs/PHASE_16_REAL_RELEASE_PLAN.md
test -f tools/release/verify_release_assets.py
test -f tools/release/bump_version.sh
test -f .github/workflows/release.yml

# Ensure workflow still expects tag input and includes latest.json generation (Phase 14)
rg -n "workflow_dispatch" .github/workflows/release.yml >/dev/null
rg -n "inputs:" .github/workflows/release.yml >/dev/null
rg -n "tag:" .github/workflows/release.yml >/dev/null
rg -n "mk_latest_json\.sh" .github/workflows/release.yml >/dev/null

echo "PHASE16_GATE_OK"
