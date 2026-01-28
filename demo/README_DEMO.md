# Execution-Lab Demo (Deterministic)

Goal: run a deterministic replay demo and produce an evidence bundle (hashes included).

1) Build release binary:
   cargo build --release -p app

2) Run demo (no cargo run):
   ./run_app.sh demo

3) Produce evidence bundle (NO python):
   ./run_app.sh bundle

4) Inspect bundle:
   ls -la demo/out/last_run
   cat demo/out/last_run/manifest.json
   cat demo/out/last_run/sha256.txt

Optional (legacy):
   python3 demo/mk_bundle.py
