#!/usr/bin/env bash
set -euo pipefail

echo "=== SMOKE PATCH ==="
date
pwd

echo "ok $(date)" > patches/_smoke_ok.txt

echo "=== END SMOKE PATCH ==="

