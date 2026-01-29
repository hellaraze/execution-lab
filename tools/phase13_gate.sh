#!/usr/bin/env bash
set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

test -f docs/PHASE_13_UPDATE_FEED_SEALED.md
test -f tools/release/README_PHASE13_UPDATE_FEED.md

test -f tools/release/repo_slug.sh
test -f tools/release/updater_endpoint_template.sh
test -f tools/release/sign_update_manifest.sh
test -f tools/release/verify_update_manifest.sh

CONF="el_gui/src-tauri/tauri.conf.json"
test -f "$CONF"

rg -n '"updater"\s*:\s*\{' "$CONF" >/dev/null
rg -n 'github\.com/.*/releases/download/v\{\{current_version\}\}/update\.json' "$CONF" >/dev/null

WF=".github/workflows/release.yml"
test -f "$WF"
rg -n "update\.json\.sig" "$WF" >/dev/null

echo "PHASE13_GATE_OK"
