#!/usr/bin/env bash
set -e
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

test -f docs/PHASE_14_REAL_CI_SIGNING_SEALED.md
test -f tools/release/README_PHASE14_REAL_CI_SIGNING.md
test -f tools/release/mk_latest_json.sh
test -f keys/tauri_updater.key.pub

rg -n '^/keys/tauri_updater\.key$' .gitignore >/dev/null

CONF="el_gui/src-tauri/tauri.conf.json"
test -f "$CONF"
rg -n '"createUpdaterArtifacts"\s*:\s*true' "$CONF" >/dev/null
rg -n 'releases/latest/download/latest\.json' "$CONF" >/dev/null
rg -n '"pubkey"\s*:\s*"' "$CONF" >/dev/null
! rg -n 'REPLACE_WITH_' "$CONF" >/dev/null

WF=".github/workflows/release.yml"
test -f "$WF"
rg -n 'TAURI_SIGNING_PRIVATE_KEY' "$WF" >/dev/null
rg -n 'mk_latest_json\.sh' "$WF" >/dev/null
rg -n 'dist/release/latest\.json' "$WF" >/dev/null

echo "PHASE14_GATE_OK"
