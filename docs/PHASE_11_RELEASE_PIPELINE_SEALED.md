# Phase 11 (Release Pipeline / Artifact Publishing / Update Feed) — Sealed

Goal: CI skeleton that produces publishable artifacts and a deterministic "update feed" placeholder.

Delivered:
- `.github/workflows/release.yml` (workflow_dispatch, windows-latest, build via Phase 7 script, draft release)
- `tools/release/version.sh`
- `tools/release/mk_release_notes.sh`
- `tools/release/mk_update_manifest.sh`
- `tools/phase11_gate.sh`
