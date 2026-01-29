#!/usr/bin/env bash
set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

MANIFEST="${1:-dist/release/update.json}"
SIG="${2:-dist/release/update.json.sig}"
PUBKEY="${3:-keys/update_public.pem}"

test -f "$MANIFEST"
test -f "$SIG"
if [ ! -f "$PUBKEY" ]; then
  echo "ERROR: missing public key: $PUBKEY"
  echo "Derive one (example): openssl pkey -in keys/update_private.pem -pubout -out keys/update_public.pem"
  exit 1
fi

openssl dgst -sha256 -verify "$PUBKEY" -signature "$SIG" "$MANIFEST"
echo "VERIFY_OK"
