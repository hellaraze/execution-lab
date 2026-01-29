# Phase 14 (Real CI Signing / No Placeholders) — Sealed

Delivered:
- `bundle.createUpdaterArtifacts=true`
- updater endpoints -> GitHub Releases `latest.json`
- updater pubkey embedded (real)
- CI wired for real signing secrets + publishes `latest.json`
- keys discipline: private key ignored, public key committed
- gate: `tools/phase14_gate.sh`
