#!/usr/bin/env bash
set -euo pipefail

NAME="${1:-}"
if [[ -z "$NAME" ]]; then
  echo "usage: ./mkpatch.sh NNN_name"
  exit 2
fi

FILE="patches/${NAME}.sh"

mkdir -p patches

if [[ -f "$FILE" ]]; then
  echo "exists: $FILE"
  exit 2
fi

cat > "$FILE"
chmod +x "$FILE"

echo "written: $FILE"
./apply.sh "$NAME"
