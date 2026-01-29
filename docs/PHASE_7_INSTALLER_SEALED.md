# Phase 7 (Installer / Distribution) — Sealed

Goal: buyer-grade distribution artifacts for Windows.

## What this phase provides
- Windows build script (PowerShell) that builds Tauri bundles and copies them into:
  - `dist/phase7/windows/bundle/*`
- Double-click entrypoint:
  - `tools/win/build_installer.cmd`

## How to run (Windows)
Open PowerShell **on Windows** and run:
- `tools\win\build_installer.ps1`

Or double-click:
- `tools\win\build_installer.cmd`

## Expected outputs
- MSI (if enabled by Tauri): `dist\phase7\windows\bundle\msi\*.msi`
- NSIS EXE (if enabled by Tauri): `dist\phase7\windows\bundle\nsis\*.exe`

## Prerequisites (Windows)
- Rust toolchain (MSVC)
- Node.js (npm) or pnpm/yarn (depending on lockfile)
- Tauri prerequisites for Windows
- WebView2 runtime (typically already installed)

## Notes
This phase is about **distribution mechanics** (installer artifacts + repeatable script),
not about code-signing or auto-updaters. Those are later phases.
