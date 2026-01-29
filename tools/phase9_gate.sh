#!/usr/bin/env bash
set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

test -f docs/PHASE_9_SIGNING_SEALED.md
test -f tools/release/README_PHASE9_SIGNING.md
test -f tools/win/sign_artifacts.ps1
test -f tools/win/sign_artifacts.cmd

# basic content markers
rg -n "signtool" tools/win/sign_artifacts.ps1 >/dev/null
rg -n "CertThumbprint" tools/win/sign_artifacts.ps1 >/dev/null
rg -n "PfxPath" tools/win/sign_artifacts.ps1 >/dev/null
rg -n "SIGN_OK" tools/win/sign_artifacts.ps1 >/dev/null

echo "PHASE9_GATE_OK"
