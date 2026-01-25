#!/usr/bin/env bash
set -euo pipefail
rg -n "error\[|error:|FAILED|panicked|thread '.*' panicked" logs/_latest.log || true
