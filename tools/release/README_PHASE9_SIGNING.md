# Phase 9 — Code Signing / Release Identity (skeleton)

Purpose:
- Establish a repeatable, verifiable signing step for Windows artifacts (MSI/EXE).
- Provide a buyer-grade trust hook: "this binary is signed and verifiable".

## What this phase adds
- `tools/win/sign_artifacts.ps1`:
  - Signs `.msi` and `.exe` under an input directory (defaults to `dist\phase7\windows\bundle`)
  - Supports either:
    - `-CertThumbprint` (certificate installed in Windows cert store), OR
    - `-PfxPath` (+ optional `-PfxPassword`)
  - Verifies signatures after signing via `signtool verify`

- `tools/win/sign_artifacts.cmd` double-click wrapper.

## Not included yet (later phases)
- Real EV cert procurement
- CI/CD signing pipeline + secrets management
- Update feed signing + release manifests
