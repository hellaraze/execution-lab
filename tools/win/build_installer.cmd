@echo off
setlocal enabledelayedexpansion
set PS=%SystemRoot%\System32\WindowsPowerShell\v1.0\powershell.exe
"%PS%" -NoProfile -ExecutionPolicy Bypass -File "%~dp0\build_installer.ps1"
set EC=%ERRORLEVEL%
echo EXIT CODE: %EC%
exit /b %EC%
