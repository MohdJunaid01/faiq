@echo off
if exist "%~dp0target\release\faiq.exe" (
    "%~dp0target\release\faiq.exe" %*
) else if exist "%~dp0target\debug\faiq.exe" (
    "%~dp0target\debug\faiq.exe" %*
) else (
    cargo run --release -- %*
)

