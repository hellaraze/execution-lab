#!/usr/bin/env bash
set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "=== LEVEL 3 GATE ==="
echo "ts:  $(date -Iseconds)"
echo "pwd: $PWD"
echo

echo "=== TOOLCHAIN ==="
rustc -V
cargo -V
echo

echo "=== GIT STATUS (informational) ==="
git status --short || true
echo

echo "=== FORMAT (CHECK) ==="
cargo fmt --check
echo "OK: fmt"
echo

echo "=== CLIPPY (workspace, all targets, deny warnings) ==="
cargo clippy -q --workspace --all-targets -- -D warnings
echo "OK: clippy"
echo

echo "=== TESTS (workspace) ==="
cargo test -q --workspace
echo "OK: tests workspace"
echo

echo "=== TESTS (workspace, all features) ==="
cargo test -q --workspace --all-features
echo "OK: tests all-features"
echo

echo "=== TESTS (workspace, all features, release) ==="
cargo test -q --workspace --all-features --release
echo "OK: tests release all-features"
echo

echo "=== SMOKE (help should not crash) ==="
cargo run -q -p elctl -- --help >/dev/null
cargo run -q -p d2 --features replay-ro --bin d2_scan -- --help >/dev/null
echo "OK: smoke"
echo

echo "=== LEVEL 3 GATE: PASS ==="
