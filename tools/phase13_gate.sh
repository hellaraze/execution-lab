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

# updater must exist and point to GitHub Releases feed
rg -n '"updater"\s*:\s*\{' "$CONF" >/dev/null
if rg -n 'releases/latest/download/latest\.json' "$CONF" >/dev/null; then
  : # Phase 14+ style
elif rg -n 'releases/download/v\{\{current_version\}\}/update\.json' "$CONF" >/dev/null; then
  : # Phase 13 style
else
  echo "ERROR: updater endpoints not wired to GitHub Releases (update.json or latest.json)"
  exit 1
fi

WF=".github/workflows/release.yml"
test -f "$WF"

# workflow must create some updater feed artifact (phase13 or phase14+)
if rg -n 'mk_latest_json\.sh' "$WF" >/dev/null; then
  rg -n 'dist/release/latest\.json' "$WF" >/dev/null
elif rg -n 'mk_update_manifest\.sh' "$WF" >/dev/null; then
  rg -n 'dist/release/update\.json' "$WF" >/dev/null
else
  echo "ERROR: release workflow does not generate updater feed (mk_latest_json or mk_update_manifest)"
  exit 1
fi

echo "PHASE13_GATE_OK"
