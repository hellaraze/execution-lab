# EXECUTION-LAB - REPRODUCIBILITY (BASELINE)

This baseline is designed to be reproducible from source.
The goal is to allow an independent engineer to reproduce deterministic outputs.

---

## Prerequisites

- Rust toolchain pinned by `rust-toolchain.toml`
- Standard build dependencies for Rust on the target OS

---

## One-Command Reproduction

Run from the repository root:

1) Execute the baseline repro script:

    ./tools/repro_baseline.sh

2) The script writes a single log file:

    repro_baseline_<UTC_TIMESTAMP>.log

3) Verify the log includes:

- toolchain versions
- git commit hash
- quality gate results
- deterministic demo outputs (D2 scan + golden test)

---

## Notes

- The repro script is intentionally self-contained.
- It prints warnings if optional fixtures are not present.

