$ErrorActionPreference = "Stop"

# resolve repo root from this script location
$Root = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
Write-Host "ROOT=$Root"

# NOTE:
# - Run this script from Windows (PowerShell), not inside WSL.
# - Requires: Rust (MSVC), Node.js, WebView2 runtime (usually already), and Tauri deps.
# - Output: dist\phase7\windows\bundle\...

Set-Location $Root

# best-effort detect Tauri dir
$TauriDir = $null
if (Test-Path (Join-Path $Root "app\src-tauri")) { $TauriDir = (Join-Path $Root "app") }
elseif (Test-Path (Join-Path $Root "src-tauri")) { $TauriDir = $Root }
else {
  $candidate = Get-ChildItem -Path $Root -Directory -ErrorAction SilentlyContinue |
    Where-Object { Test-Path (Join-Path $_.FullName "src-tauri") } |
    Select-Object -First 1
  if ($candidate) { $TauriDir = $candidate.FullName }
}
if (-not $TauriDir) { throw "Cannot locate Tauri project directory (src-tauri not found)." }

Write-Host "TAURI_DIR=$TauriDir"
Set-Location $TauriDir

# install JS deps (supports npm/pnpm/yarn; default npm)
if (Test-Path (Join-Path $TauriDir "pnpm-lock.yaml")) {
  if (-not (Get-Command pnpm -ErrorAction SilentlyContinue)) { throw "pnpm not found; install pnpm or delete pnpm-lock.yaml" }
  pnpm install
  pnpm tauri build
} elseif (Test-Path (Join-Path $TauriDir "yarn.lock")) {
  if (-not (Get-Command yarn -ErrorAction SilentlyContinue)) { throw "yarn not found; install yarn or delete yarn.lock" }
  yarn install
  yarn tauri build
} else {
  if (-not (Get-Command npm -ErrorAction SilentlyContinue)) { throw "npm not found; install Node.js" }
  npm ci
  npx tauri build
}

# copy bundles to dist/phase7/windows (repo root)
$Out = (Join-Path $Root "dist\phase7\windows")
New-Item -ItemType Directory -Force -Path $Out | Out-Null

# Tauri outputs under src-tauri\target\release\bundle\...
$Bundle = Join-Path $TauriDir "src-tauri\target\release\bundle"
if (-not (Test-Path $Bundle)) { throw "Bundle dir not found: $Bundle" }

Copy-Item -Recurse -Force $Bundle (Join-Path $Out "bundle")

Write-Host "OK: copied bundles to $Out\bundle"
Write-Host "TIP: look for .msi under $Out\bundle\msi and .exe under $Out\bundle\nsis (if enabled)"
