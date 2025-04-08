@echo off
echo =======================================
echo      Installing Text Expander
echo =======================================

set APP_NAME=text_expander
set INSTALL_DIR=%LOCALAPPDATA%\TextExpander
set SHORTCUT_PATH=%USERPROFILE%\Desktop\TextExpander.lnk
set ICON_PATH=%INSTALL_DIR%\%APP_NAME%.ico
set EXE_PATH=%INSTALL_DIR%\%APP_NAME%.exe

echo 📁 Creating install folder: %INSTALL_DIR%
mkdir "%INSTALL_DIR%" 2>nul

echo 📦 Copying files...
copy /Y "%APP_NAME%.exe" "%INSTALL_DIR%\"
copy /Y "%APP_NAME%.ico" "%INSTALL_DIR%\"

echo 🧪 Verifying copied files...
if not exist "%EXE_PATH%" (
    echo ❌ ERROR: %APP_NAME%.exe not found at %EXE_PATH%
    pause
    exit /b
)
if not exist "%ICON_PATH%" (
    echo ❌ ERROR: %APP_NAME%.ico not found at %ICON_PATH%
    pause
    exit /b
)

echo 🔗 Creating desktop shortcut...
powershell -Command ^
  "$s = (New-Object -COM WScript.Shell).CreateShortcut('%SHORTCUT_PATH%'); $s.TargetPath = '%EXE_PATH%'; $s.IconLocation = '%ICON_PATH%'; $s.Save()"

echo ✅ Text Expander installed successfully!
echo 🖼️  Shortcut created on your desktop.
pause
