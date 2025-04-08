@echo off
setlocal

set APP_NAME=Text Expander
set EXE_NAME=text_expander.exe
set ICO_NAME=text_expander.ico
set INSTALL_DIR=%LOCALAPPDATA%\TextExpander
set SHORTCUT_PATH=%USERPROFILE%\Desktop\TextExpander.lnk

echo 🛠️ Installing %APP_NAME%...

:: Create install directory
if not exist "%INSTALL_DIR%" (
    mkdir "%INSTALL_DIR%"
)

:: Copy files to install directory
copy /Y "%~dp0%EXE_NAME%" "%INSTALL_DIR%"
copy /Y "%~dp0%ICO_NAME%" "%INSTALL_DIR%"

:: Create shortcut on desktop using PowerShell
powershell -Command ^
  "$s = (New-Object -COM WScript.Shell).CreateShortcut('%SHORTCUT_PATH%'); ^
   $s.TargetPath = '%INSTALL_DIR%\%EXE_NAME%'; ^
   $s.IconLocation = '%INSTALL_DIR%\%ICO_NAME%'; ^
   $s.Save()"

echo ✅ %APP_NAME% installed successfully!
echo 📌 Shortcut created on your desktop.

pause

