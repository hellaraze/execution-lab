#!/usr/bin/env bash
set -euo pipefail
N="${1:-200}"
tail -n "$N" logs/_latest.log
