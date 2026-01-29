#!/usr/bin/env bash
set -e
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
test -f .github/workflows/release.yml
test -f docs/PHASE_11_RELEASE_PIPELINE_SEALED.md
test -f tools/release/version.sh
test -f tools/release/mk_release_notes.sh
test -f tools/release/mk_update_manifest.sh
rg -n "workflow_dispatch" .github/workflows/release.yml >/dev/null
rg -n "windows-latest" .github/workflows/release.yml >/dev/null
rg -n "action-gh-release" .github/workflows/release.yml >/dev/null
rg -n "build_installer\.ps1" .github/workflows/release.yml >/dev/null
VER="$(tools/release/version.sh)"
test -n "$VER"
echo "PHASE11_GATE_OK"
