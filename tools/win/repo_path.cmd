@echo off
for /f "usebackq tokens=*" %%i in (`wsl.exe wslpath -w "%~dp0\..\.."`) do set REPO=%%i
echo %REPO%
