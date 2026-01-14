#!/bin/bash
echo "Building GdSerial for release..."

# Detect platform and architecture
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture names
case $ARCH in
    x86_64) ARCH_NAME="x86_64" ;;
    arm64|aarch64) ARCH_NAME="arm64" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

echo "Building for platform: $PLATFORM, architecture: $ARCH_NAME"

# Build the Rust library
cargo build --release

# Create platform-specific bin directories
case $PLATFORM in
    linux)
        mkdir -p "addons/gdserial/bin/linux-$ARCH_NAME"
        cp target/release/libgdserial.so "addons/gdserial/bin/linux-$ARCH_NAME/" 2>/dev/null || true
        echo "Linux library copied to addons/gdserial/bin/linux-$ARCH_NAME/"
        ;;
    darwin)
        mkdir -p "addons/gdserial/bin/macos-$ARCH_NAME"
        cp target/release/libgdserial.dylib "addons/gdserial/bin/macos-$ARCH_NAME/" 2>/dev/null || true
        echo "macOS library copied to addons/gdserial/bin/macos-$ARCH_NAME/"
        ;;
    *)
        echo "Unsupported platform: $PLATFORM"
        exit 1
        ;;
esac

echo "Build complete! Library files copied to addons/gdserial/bin/"
echo ""

# Copy addon to example project
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
bash "$SCRIPT_DIR/copy_addon.sh"