$ErrorActionPreference = "Stop"

param(
  [string]$InputDir = "",
  [string]$PfxPath = "",
  [string]$PfxPassword = "",
  [string]$CertThumbprint = "",
  [string]$TimestampUrl = "http://timestamp.digicert.com"
)

function Find-Signtool {
  $candidates = @(
    "${env:ProgramFiles(x86)}\Windows Kits\10\bin\x64\signtool.exe",
    "${env:ProgramFiles(x86)}\Windows Kits\10\bin\x86\signtool.exe",
    "${env:ProgramFiles}\Windows Kits\10\bin\x64\signtool.exe",
    "${env:ProgramFiles}\Windows Kits\10\bin\x86\signtool.exe"
  )
  foreach ($p in $candidates) { if (Test-Path $p) { return $p } }
  $cmd = Get-Command signtool.exe -ErrorAction SilentlyContinue
  if ($cmd) { return $cmd.Path }
  throw "signtool.exe not found. Install Windows SDK (Windows Kits)."
}

function Require-OneSigner {
  if ([string]::IsNullOrWhiteSpace($CertThumbprint) -and [string]::IsNullOrWhiteSpace($PfxPath)) {
    throw "Provide either -CertThumbprint (installed cert) OR -PfxPath (+ optional -PfxPassword)."
  }
}

function Sign-OneFile([string]$signtool, [string]$file) {
  Write-Host "SIGN: $file"

  if (-not [string]::IsNullOrWhiteSpace($PfxPath)) {
    if (-not (Test-Path $PfxPath)) { throw "PFX not found: $PfxPath" }
    $args = @("sign","/fd","SHA256","/tr",$TimestampUrl,"/td","SHA256","/f",$PfxPath)
    if (-not [string]::IsNullOrWhiteSpace($PfxPassword)) { $args += @("/p",$PfxPassword) }
    $args += @($file)
    & $signtool @args
  } else {
    # thumbprint-based signing from cert store
    $args = @("sign","/fd","SHA256","/tr",$TimestampUrl,"/td","SHA256","/sha1",$CertThumbprint,$file)
    & $signtool @args
  }
}

function Verify-OneFile([string]$signtool, [string]$file) {
  Write-Host "VERIFY: $file"
  & $signtool "verify" "/pa" "/v" $file
}

# Determine default input dir (repo dist)
if ([string]::IsNullOrWhiteSpace($InputDir)) {
  $Root = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
  $InputDir = (Join-Path $Root "dist\phase7\windows\bundle")
}

$InputDir = (Resolve-Path $InputDir).Path
Write-Host "INPUT=$InputDir"

Require-OneSigner
$signtool = Find-Signtool
Write-Host "SIGNTOOL=$signtool"

# Collect candidates (MSI + EXE)
$files = @()
$files += Get-ChildItem -Path $InputDir -Recurse -File -Include *.msi -ErrorAction SilentlyContinue
$files += Get-ChildItem -Path $InputDir -Recurse -File -Include *.exe -ErrorAction SilentlyContinue

if ($files.Count -eq 0) {
  Write-Host "NO_ARTIFACTS_FOUND (nothing to sign)"
  exit 0
}

foreach ($f in $files) {
  Sign-OneFile $signtool $f.FullName
  Verify-OneFile $signtool $f.FullName
}

Write-Host "SIGN_OK"
