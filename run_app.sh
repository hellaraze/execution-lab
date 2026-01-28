#!/usr/bin/env bash
set -e
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

if [ -x target/release/app ]; then
  exec target/release/app "$@"
fi
if [ -x target/debug/app ]; then
  exec target/debug/app "$@"
fi

echo "app binary not found; build first:"
echo "  cargo build --release -p app"
exit 2
