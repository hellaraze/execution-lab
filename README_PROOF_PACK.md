# Proof Pack (Phase 12)

Proof Pack is a single file you can hand to a buyer/auditor:
- runs/<run_id>/proof_pack.tar.gz

Create run (replay):
  cargo run -p elctl --bin execution-lab -- run --config configs/replay.toml

Create proof pack:
  cargo run -p elctl --bin execution-lab -- proof-pack --run-dir runs/<run_id>

Artifacts inside run_dir:
- proof/toolchain.txt
- proof/env.txt
- proof/proof_manifest.json
- proof_pack.tar.gz
