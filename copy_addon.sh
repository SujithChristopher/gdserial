#!/bin/bash
# Copy addon to example folder after build
# This script is called by build_release.sh

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
SOURCE_ADDON="$SCRIPT_DIR/addons/gdserial"
DEST_ADDON="$SCRIPT_DIR/example/addons/gdserial"

echo ""
echo "========================================"
echo "Copying addon to example project..."
echo "========================================"

# Create destination directory if it doesn't exist
mkdir -p "$DEST_ADDON"

echo "Copying from: $SOURCE_ADDON"
echo "Copying to:   $DEST_ADDON"

# Copy entire addon folder
cp -r "$SOURCE_ADDON"/* "$DEST_ADDON/" 2>/dev/null

if [ $? -eq 0 ]; then
    echo ""
    echo "✓ Addon copied successfully!"
    echo "You can now open the example project in Godot"
else
    echo ""
    echo "✗ Error copying addon"
    exit 1
fi
