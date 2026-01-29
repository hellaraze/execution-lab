#!/usr/bin/env bash
set -e
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
VER="$(tools/release/version.sh)"
OUT="${1:-dist/release/update.json}"
mkdir -p "$(dirname "$OUT")"
cat > "$OUT" <<EOF2
{
  "product": "execution-lab",
  "version": "$VER",
  "published_at": "$(date -Iseconds)",
  "notes": "placeholder update manifest (Phase 11 skeleton)",
  "targets": {
    "windows-x86_64": {
      "url": "https://github.com/' .  . '/releases/download/v$VER/installer.exe",
      "signature": "REPLACE_WITH_REAL_SIGNATURE"
    }
  }
}
EOF2
echo "WROTE: $OUT"
