# Buyer Quickstart (offline + trust)

## What you get
- Signed Windows installer (when vendor provides signing key in CI)
- Auto-update feed via GitHub Releases (`latest.json`)
- Proof Pack (buyer evidence bundle) in-repo docs

## Fast path (demo/proof)
1) Install the app (Windows installer artifact from a Release).
2) Launch **Execution Lab**.
3) Use the built-in demo/proof path (Phase 12 Proof Pack):
   - `README_PROOF_PACK.md` and the proof artifacts.
4) Verify update channel:
   - updater endpoint: `.../releases/latest/download/latest.json`

## Trust checks
- Installer should be code-signed (Phase 9 skeleton; Phase 14 CI signing for updater artifacts).
- Updater verifies signatures using embedded pubkey in `tauri.conf.json`.
