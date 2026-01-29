#!/usr/bin/env bash
set -e
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

OUT="${1:-dist/release/latest.json}"
mkdir -p "$(dirname "$OUT")"

VER="$(tr -d ' \t\r\n' < VERSION)"
TAG="v$VER"

SLUG="$(tools/release/repo_slug.sh 2>/dev/null || true)"
if [ -z "$SLUG" ] || [ "$SLUG" = "UNKNOWN/UNKNOWN" ]; then SLUG="hellaraze/execution-lab"; fi

EXE="$(ls -1 dist/phase7/windows/bundle/nsis/*.exe 2>/dev/null | head -n1 || true)"
if [ -n "$EXE" ]; then ART="$EXE"; else ART="$(ls -1 dist/phase7/windows/bundle/msi/*.msi 2>/dev/null | head -n1 || true)"; fi

test -f "$ART"
SIG="${ART}.sig"
test -f "$SIG"

SIG_JSON="$(python3 - "$SIG" <<'PY2'
import json,sys
print(json.dumps(open(sys.argv[1],"r",encoding="utf-8",errors="replace").read()))
PY2
)"

FN="$(basename "$ART")"
URL="https://github.com/${SLUG}/releases/download/${TAG}/${FN}"
PUB="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

cat > "$OUT" <<EOF2
{
  "version": "$TAG",
  "notes": "Automated release (Phase 14: real Tauri updater signing)",
  "pub_date": "$PUB",
  "platforms": {
    "windows-x86_64": {
      "signature": $SIG_JSON,
      "url": "$URL"
    }
  }
}
EOF2

echo "WROTE: $OUT"
