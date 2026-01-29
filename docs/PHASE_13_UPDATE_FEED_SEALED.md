# Phase 13 (Update Feed / Signature / Endpoint Wiring) — Sealed

Goal: switch updater from placeholder endpoints to a real distribution location (GitHub Releases)
and define a verifiable signature contract for update manifests.

Delivered:
- Updater endpoint wired to GitHub Releases template:
  - `.../releases/download/v{{current_version}}/update.json`
- Signing contract scripts:
  - `tools/release/sign_update_manifest.sh`
  - `tools/release/verify_update_manifest.sh`
- Repo slug detection:
  - `tools/release/repo_slug.sh`
  - `tools/release/updater_endpoint_template.sh`
- CI workflow extended to include `update.json.sig` placeholder asset.
- Gate: `tools/phase13_gate.sh`
