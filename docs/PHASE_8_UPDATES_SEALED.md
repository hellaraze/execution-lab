# Phase 8 (Updates / Versioning / Trust) — Sealed

Goal: make the product update-ready and trust-ready at the config/contract level.

Delivered:
- `VERSION`
- Tauri v2 updater wiring in `el_gui/src-tauri/tauri.conf.json`:
  - `plugins.updater.active=true`
  - endpoints template present
  - pubkey placeholder present
- `tools/release/README_PHASE8_UPDATES.md`
- `tools/phase8_gate.sh`
