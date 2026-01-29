@echo off
setlocal enabledelayedexpansion
set PS=%SystemRoot%\System32\WindowsPowerShell\v1.0\powershell.exe
REM Usage examples:
REM   tools\win\sign_artifacts.cmd -CertThumbprint "ABCD..." -InputDir "dist\phase7\windows\bundle"
REM   tools\win\sign_artifacts.cmd -PfxPath "C:\path\cert.pfx" -PfxPassword "..." -InputDir "dist\phase7\windows\bundle"
"%PS%" -NoProfile -ExecutionPolicy Bypass -File "%~dp0\sign_artifacts.ps1" %*
set EC=%ERRORLEVEL%
echo EXIT CODE: %EC%
exit /b %EC%
