@echo off
setlocal

echo ╔════════════════════════════════════╗
echo ║     Installing Text Expander       ║
echo ╚════════════════════════════════════╝

REM Set install path
set INSTALL_DIR=%USERPROFILE%\AppData\Local\TextExpander

REM Create install folder
mkdir "%INSTALL_DIR%" >nul 2>&1

REM Copy files
copy text_expander.exe "%INSTALL_DIR%"
copy text_expander.ico "%INSTALL_DIR%"

REM Create desktop shortcut using PowerShell
powershell -Command "$s = (New-Object -ComObject WScript.Shell).CreateShortcut('%USERPROFILE%\Desktop\TextExpander.lnk'); $s.TargetPath = '%INSTALL_DIR%\text_expander.exe'; $s.IconLocation = '%INSTALL_DIR%\text_expander.ico'; $s.Save()"

echo 🟢 Text Expander installed successfully!
echo 🖥️  Shortcut created on your desktop.
pause
