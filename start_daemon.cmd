@echo off
echo Starting FAIQ Background Daemon on port 4242...
if exist "%~dp0target\release\faiq.exe" (
    start "FAIQ Daemon" /min "%~dp0target\release\faiq.exe" daemon --port 4242
) else (
    start "FAIQ Daemon" /min "%~dp0target\debug\faiq.exe" daemon --port 4242
)
echo FAIQ Daemon started in background.
echo Local Wi-Fi Sync endpoint: http://0.0.0.0:4242
pause

