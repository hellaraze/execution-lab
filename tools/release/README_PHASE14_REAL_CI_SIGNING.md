# Phase 14 — Real CI Signing (Tauri updater) + Keys Discipline

- Private signing key is NOT in repo: `keys/tauri_updater.key` is gitignored.
- Public key IS in repo: `keys/tauri_updater.key.pub` and embedded in Tauri config.

Updater endpoint:
- `https://github.com/<owner>/<repo>/releases/latest/download/latest.json`

CI secrets required:
- `TAURI_SIGNING_PRIVATE_KEY`
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` (optional)

CI output:
- installer (`.exe` or `.msi`) + `.sig`
- `dist/release/latest.json` built from the `.sig` content
