#!/usr/bin/env bash
set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

test -f .github/workflows/release.yml
test -f docs/PHASE_11_RELEASE_PIPELINE_SEALED.md

test -f tools/release/version.sh
test -f tools/release/mk_release_notes.sh

# accept either feed generator existing in repo (phase11: update.json, phase14+: latest.json)
if [ -f tools/release/mk_latest_json.sh ]; then
  :
elif [ -f tools/release/mk_update_manifest.sh ]; then
  :
else
  echo "ERROR: missing mk_latest_json.sh and mk_update_manifest.sh"
  exit 1
fi

WF=".github/workflows/release.yml"

# core workflow invariants (buyer-grade):
rg -n "workflow_dispatch" "$WF" >/dev/null
rg -n "uses:\s*actions/checkout@v" "$WF" >/dev/null
rg -n "build_installer\.ps1" "$WF" >/dev/null
rg -n "softprops/action-gh-release@v" "$WF" >/dev/null

# must publish some updater feed artifact reference
if rg -n "mk_latest_json\.sh" "$WF" >/dev/null; then
  rg -n "dist/release/latest\.json" "$WF" >/dev/null
elif rg -n "mk_update_manifest\.sh" "$WF" >/dev/null; then
  rg -n "dist/release/update\.json" "$WF" >/dev/null
else
  echo "ERROR: workflow does not generate updater feed (mk_latest_json or mk_update_manifest)"
  exit 1
fi

VER="$(tools/release/version.sh)"
test -n "$VER"

echo "PHASE11_GATE_OK"
