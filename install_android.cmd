@echo off
set ADB="%LOCALAPPDATA%\Android\Sdk\platform-tools\adb.exe"
set APK="%~dp0android\app\build\outputs\apk\debug\app-debug.apk"

if not exist %ADB% (
    set ADB=adb
)

if not exist %APK% (
    echo Error: APK not found at %APK%
    echo Please run assembleDebug first.
    pause
    exit /b 1
)

echo Checking for connected Android devices...
%ADB% devices

echo Installing FAIQ Companion App...
%ADB% install -r %APK%

if %ERRORLEVEL% equ 0 (
    echo.
    echo Success! FAIQ has been installed on your device.
) else (
    echo.
    echo Installation failed. Ensure your phone is connected with USB Debugging enabled.
)
pause

