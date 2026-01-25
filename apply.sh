#!/usr/bin/env bash
set -euo pipefail

NAME="${1:-}"
if [[ -z "$NAME" ]]; then
  echo "usage: ./apply.sh <patch_name_without_.sh>"
  echo "example: ./apply.sh 001_fix_paths"
  exit 2
fi

PATCH="patches/${NAME}.sh"
if [[ ! -f "$PATCH" ]]; then
  echo "patch not found: $PATCH"
  exit 2
fi

echo "== APPLY $PATCH =="
bash "$PATCH"

echo "== VERIFY =="
./run_block.sh
