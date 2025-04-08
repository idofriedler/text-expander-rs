#!/bin/bash

set -e

APP_NAME="text_expander"
INSTALL_DIR="$HOME/.local/bin"
DESKTOP_FILE="$HOME/.local/share/applications/${APP_NAME}.desktop"
ICON_SRC="$INSTALL_DIR/text_expander.png"
ICON_DEST="$HOME/.local/share/icons/text_expander.png"

# Create install dir if needed
echo "📁 Ensuring install dir exists: $INSTALL_DIR"
mkdir -p "$INSTALL_DIR"

# Check if binary zip exists
if [[ ! -f "./text_expander.zip" ]]; then
    echo "❌ text_expander.zip not found. Please download or build it first."
    exit 1
fi

# Extract the zip
echo "📦 Extracting app to $INSTALL_DIR..."
unzip -o text_expander.zip -d "$INSTALL_DIR"

# Ensure binary is executable
chmod +x "$INSTALL_DIR/$APP_NAME"
echo "✅ Binary installed to $INSTALL_DIR/$APP_NAME"

# Create .desktop entry
echo "📝 Creating desktop shortcut at $DESKTOP_FILE..."
cat > "$DESKTOP_FILE" <<EOL
[Desktop Entry]
Name=Text Expander
Exec=${INSTALL_DIR}/${APP_NAME}
Icon=${ICON_DEST}
Type=Application
Terminal=false
Categories=Utility;
EOL

# Copy icon if it exists
if [[ -f "$ICON_SRC" ]]; then
    mkdir -p "$(dirname "$ICON_DEST")"
    cp "$ICON_SRC" "$ICON_DEST"
    echo "🖼️  Icon copied to $ICON_DEST"
else
    echo "⚠️  Icon source file missing: $ICON_SRC"
fi

# Make .desktop executable
chmod +x "$DESKTOP_FILE"
echo "🎯 Shortcut created at $DESKTOP_FILE"

# Done
echo "✅ Text Expander installed successfully!"
echo "📍 You can find it in your app menu now."
