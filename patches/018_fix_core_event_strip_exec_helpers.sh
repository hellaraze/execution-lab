#!/usr/bin/env bash
set -euo pipefail

# Strip any leftover exec-specific helpers from core/src/event.rs
perl -0777 -i -pe '
s/\n\s*\| ExecEvent::OrderValidated[\s\S]*?\}\n\}\n//s
' core/src/event.rs

echo "core/src/event.rs stripped of exec helpers"
