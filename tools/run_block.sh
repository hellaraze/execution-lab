#!/usr/bin/env bash
set -e
BLOCK="${1:-}"
if [ -z "$BLOCK" ] || [ ! -f "$BLOCK" ]; then
  echo "USAGE: tools/run_block.sh /path/to/block.sh"
  exit 2
fi
bash "$BLOCK" 2>&1 | cb
