# Phase 9 (Code Signing / Release Identity / Trust Hardening) — Sealed

Goal: signing pipeline skeleton for Windows installer artifacts.

Delivered:
- `tools/win/sign_artifacts.ps1` (signtool-based signing + verify)
- `tools/win/sign_artifacts.cmd` (double-click wrapper)
- `tools/release/README_PHASE9_SIGNING.md` (process contract)
- `tools/phase9_gate.sh` (repo-level verification)

Notes:
- This is a *skeleton*: it provides the mechanism and contract.
- Real certificates, CI signing, and update-feed trust chain come later.
