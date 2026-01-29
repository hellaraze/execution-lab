#!/usr/bin/env bash
set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

test -f VERSION
test -f docs/PHASE_8_UPDATES_SEALED.md
test -f tools/release/README_PHASE8_UPDATES.md

CONF="el_gui/src-tauri/tauri.conf.json"
test -f "$CONF"

rg -n '"plugins"\s*:\s*\{' "$CONF" >/dev/null
rg -n '"updater"\s*:\s*\{' "$CONF" >/dev/null
rg -n '"active"\s*:\s*true' "$CONF" >/dev/null
rg -n '"endpoints"\s*:\s*\[' "$CONF" >/dev/null
rg -n 'REPLACE_WITH_TAURI_UPDATER_PUBLIC_KEY' "$CONF" >/dev/null

echo "PHASE8_GATE_OK"
