#!/usr/bin/env bash
set -e
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

NEW="${1:-}"
if [ -z "$NEW" ]; then
  echo "ERROR: usage: tools/release/bump_version.sh <X.Y.Z>"
  exit 1
fi
if [[ ! "$NEW" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "ERROR: not semver X.Y.Z: $NEW"
  exit 1
fi

echo "$NEW" > VERSION

python3 - <<PY2
import json, pathlib
new = "$NEW"
conf = pathlib.Path("el_gui/src-tauri/tauri.conf.json")
data = json.loads(conf.read_text(encoding="utf-8"))
data["version"] = new
conf.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")

# best-effort: update package.json version if exists
for p in [pathlib.Path("el_gui/package.json"), pathlib.Path("ui/package.json"), pathlib.Path("package.json")]:
    if p.exists():
        try:
            d = json.loads(p.read_text(encoding="utf-8"))
            d["version"] = new
            p.write_text(json.dumps(d, indent=2) + "\n", encoding="utf-8")
        except Exception:
            pass
print("bumped:", new)
PY2

echo "OK: VERSION + tauri.conf.json updated to $NEW"
