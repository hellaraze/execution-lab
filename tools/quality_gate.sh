#!/usr/bin/env bash
set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "=== PWD ==="
pwd

echo "=== TOOLCHAIN ==="
rustc -V
cargo -V

echo "=== GIT STATUS ==="
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  DIRTY="$(git status --porcelain || true)"
  if [ -n "$DIRTY" ]; then
    echo "$DIRTY"
    if [ "${ALLOW_DIRTY:-0}" != "1" ]; then
      echo "ERROR: Working tree is dirty. Commit or stash changes. (Set ALLOW_DIRTY=1 to override.)"
      exit 2
    else
      echo "WARN: ALLOW_DIRTY=1 set; proceeding despite dirty tree."
    fi
  else
    echo "OK: clean"
  fi
else
  echo "WARN: not a git repo"
fi

echo "=== FORMAT ==="
cargo fmt

echo "=== CLIPPY (workspace, all targets) ==="
cargo clippy -q --workspace --all-targets -- -D warnings

echo "=== TESTS (workspace) ==="
cargo test -q --workspace
