#!/usr/bin/env bash
set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

MANIFEST="${1:-dist/release/update.json}"
OUTSIG="${2:-dist/release/update.json.sig}"
PRIVKEY="${3:-keys/update_private.pem}"

test -f "$MANIFEST"
if [ ! -f "$PRIVKEY" ]; then
  echo "ERROR: missing private key: $PRIVKEY"
  echo "Create one (example): openssl genpkey -algorithm ed25519 -out keys/update_private.pem"
  exit 1
fi

mkdir -p "$(dirname "$OUTSIG")"

# contract: sign bytes of manifest
openssl dgst -sha256 -sign "$PRIVKEY" -out "$OUTSIG" "$MANIFEST"
echo "WROTE: $OUTSIG"
