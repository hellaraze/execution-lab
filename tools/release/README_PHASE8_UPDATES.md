# Phase 8 — Updates / Versioning / Trust (skeleton)

Tauri v2 config wiring:
- Updater lives under top-level `"plugins": { "updater": { ... } }`.
- This phase adds **placeholders** only (endpoint template + pubkey slot).

Sources of truth:
- `VERSION` (canonical release identifier)
- `el_gui/src-tauri/tauri.conf.json` (updater wiring)

Not included yet:
- real signing keys
- real update host/feed
- CI release pipeline
