@echo off
setlocal enabledelayedexpansion

REM 💬 Visual Header
echo.
echo ╔══════════════════════════════╗
echo ║   Installing Text Expander   ║
echo ╚══════════════════════════════╝
echo.

REM 📁 Define paths
set "APP_NAME=TextExpander"
set "CURDIR=%~dp0"
set "INSTALL_DIR=%LOCALAPPDATA%\%APP_NAME%"
set "DESKTOP_LINK=%USERPROFILE%\Desktop\%APP_NAME%.lnk"
set "EXE_PATH=%CURDIR%text_expander.exe"
set "ICO_PATH=%CURDIR%text_expander.ico"

REM 🔍 Check files exist
echo 🔍 Checking files...
if not exist "%EXE_PATH%" (
    echo ❌ ERROR: File not found: %EXE_PATH%
    pause
    exit /b 1
)

if not exist "%ICO_PATH%" (
    echo ❌ ERROR: File not found: %ICO_PATH%
    pause
    exit /b 1
)

REM 📦 Create install directory
echo 📂 Creating install folder: %INSTALL_DIR%
mkdir "%INSTALL_DIR%" >nul 2>&1

REM 📥 Copy files
echo 📥 Copying files...
copy /Y "%EXE_PATH%" "%INSTALL_DIR%\" >nul
copy /Y "%ICO_PATH%" "%INSTALL_DIR%\" >nul

REM 📎 Create desktop shortcut
echo 🔧 Creating desktop shortcut...
powershell -command ^
 "$s = (New-Object -COM WScript.Shell).CreateShortcut('%DESKTOP_LINK%'); ^
  $s.TargetPath = '%INSTALL_DIR%\text_expander.exe'; ^
  $s.IconLocation = '%INSTALL_DIR%\text_expander.ico'; ^
  $s.Save()"

REM ✅ Done
echo.
echo ✅ Text Expander installed successfully!
echo 📍 Shortcut created on your desktop.
pause
