# Phase 16 — Real Release Execution (first shipped release)

## Goal
Ship a real GitHub Release `vX.Y.Z` with:
- Windows installer (`.exe` or `.msi`)
- Installer signature (`.sig`) produced by Tauri when CI has signing key
- `latest.json` updater feed asset
- Release notes asset

## Required GitHub Secrets
- `TAURI_SIGNING_PRIVATE_KEY`
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` (optional)

## Operator steps
1) Bump version locally (this phase does it via script):
   - VERSION, tauri.conf.json version updated.
2) Commit version bump.
3) Create annotated tag `vX.Y.Z` on that commit.
4) Push commit + tag to origin.
5) Run GitHub Actions workflow **release** (workflow_dispatch) with input `tag=vX.Y.Z`.
6) Verify the created GitHub Release includes required assets:
   - `latest.json`
   - installer (`.exe` or `.msi`)
   - installer signature (`.sig`)
7) Mark Phase 16 verified (next phase can seal a verification doc).

## Local verification helper
- `python3 tools/release/verify_release_assets.py --slug <owner/repo> --tag vX.Y.Z`
- or `--latest` to check the latest release.
