#!/usr/bin/env bash
set -e
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
test -f VERSION
VER="$(tr -d ' \t\r\n' < VERSION)"
if [[ ! "$VER" =~ ^[0-9]+\.[0-9]+\.[0-9]+([\-+].*)?$ ]]; then
  echo "ERROR: VERSION is not semver-ish: '$VER'"
  exit 1
fi
echo "$VER"
