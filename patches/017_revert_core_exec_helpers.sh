#!/usr/bin/env bash
set -euo pipefail

# remove wrongly added impl from core/src/event.rs
perl -0777 -i -pe '
s/impl ExecEvent \{[\s\S]*?\}\n//s
' core/src/event.rs

echo "core ExecEvent helpers reverted"
