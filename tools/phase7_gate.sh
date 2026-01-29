#!/usr/bin/env bash
set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

test -f tools/win/build_installer.ps1
test -f tools/win/build_installer.cmd
test -f docs/PHASE_7_INSTALLER_SEALED.md

# sanity: tauri dir must exist somewhere
if [ ! -d app/src-tauri ] && [ ! -d src-tauri ] && [ -z "$(ls -d */src-tauri 2>/dev/null | head -n1)" ]; then
  echo "ERROR: src-tauri not found anywhere"
  exit 1
fi

echo "PHASE7_GATE_OK"
