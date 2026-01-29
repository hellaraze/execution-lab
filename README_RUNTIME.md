# Product Runtime (Phase 10)

Canonical entrypoint:
- **`execution-lab`** (single product runtime)

Everything else is dev-only.

---

## Build

Debug:
- `cargo build -p elctl --bin execution-lab`

Release:
- `cargo build -p elctl --bin execution-lab --release`

---

## Configs

Samples:
- `configs/replay.toml`
- `configs/live_dryrun.toml`
- `configs/live_postonly.toml`

Validate:
- `./target/debug/execution-lab validate-config --config configs/replay.toml`

---

## Run: replay

- `./target/debug/execution-lab run --config configs/replay.toml`

Artifacts:
- `runs/<run_id>/run_manifest.json`
- `runs/<run_id>/decisions.log`
- `runs/<run_id>/sha256.txt`
- `runs/<run_id>/replay_input_sha256.txt`
- `runs/<run_id>/RUN_OK`

---

## Run: live (dry-run / post-only)

Dry-run:
- `./target/debug/execution-lab run --config configs/live_dryrun.toml`

Post-only:
- `./target/debug/execution-lab run --config configs/live_postonly.toml`

Kill-switch (minimal):
- create file at `kill_switch.path` (default: `state/KILL`) to stop runtime.

Stop:
- Ctrl+C triggers graceful stop.
