# Phase 13 — Real Update Feed + Signature Contract

This phase wires the updater endpoint to **GitHub Releases** and defines a **manifest signing contract**.

## Update feed location (GitHub Releases)
Updater endpoint template:
- `https://github.com/<owner>/<repo>/releases/download/v{{current_version}}/update.json`

This means:
- For VERSION = `0.1.0`, tag should be `v0.1.0`
- The GitHub Release must include an asset named `update.json`

## Manifest signing (contract)
We sign the bytes of `update.json`:

- Sign:
  - `tools/release/sign_update_manifest.sh dist/release/update.json dist/release/update.json.sig keys/update_private.pem`
- Verify:
  - `tools/release/verify_update_manifest.sh dist/release/update.json dist/release/update.json.sig keys/update_public.pem`

Key generation examples:
- `openssl genpkey -algorithm ed25519 -out keys/update_private.pem`
- `openssl pkey -in keys/update_private.pem -pubout -out keys/update_public.pem`

## CI note
Current CI produces a placeholder `update.json.sig`.
Real CI signing + secret handling is a later phase.
