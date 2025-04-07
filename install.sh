#!/bin/bash

set -e

APP_NAME="text_expander"
INSTALL_DIR="$HOME/.local/bin"
DESKTOP_FILE="$HOME/.local/share/applications/${APP_NAME}.desktop"

# Create install dir if needed
mkdir -p "$INSTALL_DIR"

# Check if binary already exists in repo
if [[ ! -f "./text_expander.zip" ]]; then
    echo "❌ text_expander.zip not found. Please download or build it first."
    exit 1
fi

# Unzip the release binary
echo "📦 Extracting app..."
unzip -o text_expander.zip -d "$INSTALL_DIR"

# Ensure executable permissions
chmod +x "$INSTALL_DIR/$APP_NAME"

# Create .desktop file for Linux GUI integration
echo "📝 Creating desktop shortcut..."
cat > "$DESKTOP_FILE" <<EOL
[Desktop Entry]
Name=Text Expander
Exec=${INSTALL_DIR}/${APP_NAME}
Icon=${HOME}/.local/share/icons/text_expander.png
Type=Application
Terminal=false
Categories=Utility;
EOL

# Copy icon (if it exists)
if [[ -f "./icon.png" ]]; then
    mkdir -p "$HOME/.local/share/icons"
    cp ./icon.png "$HOME/.local/share/icons/text_expander.png"
    echo "🖼️  Icon installed."
fi

# Make desktop file executable
chmod +x "$DESKTOP_FILE"

# Notify user
echo "✅ Text Expander installed successfully!"
echo "📍 You can find it in your app menu now."

