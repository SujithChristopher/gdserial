#!/bin/bash
echo "Building GdSerial for all supported platforms..."

# Function to build for a specific target
build_target() {
    local target=$1
    local platform=$2
    local arch=$3
    local ext=$4
    local bin_dir="addons/gdserial/bin/$platform-$arch"
    
    echo "Building for $target..."
    
    # Add target if not already installed
    rustup target add $target 2>/dev/null || true
    
    # Build for the target
    if cargo build --release --target $target; then
        mkdir -p "$bin_dir"
        if [ "$platform" = "windows" ]; then
            cp "target/$target/release/gdserial.$ext" "$bin_dir/" 2>/dev/null || true
        else
            cp "target/$target/release/libgdserial.$ext" "$bin_dir/" 2>/dev/null || true
        fi
        echo "✓ Built and copied library for $target to $bin_dir/"
    else
        echo "✗ Failed to build for $target"
    fi
}

# Build for all supported targets
echo "Installing required Rust targets..."

# macOS targets
build_target "x86_64-apple-darwin" "macos" "x86_64" "dylib"
build_target "aarch64-apple-darwin" "macos" "arm64" "dylib"

# Linux targets  
build_target "x86_64-unknown-linux-gnu" "linux" "x86_64" "so"
build_target "aarch64-unknown-linux-gnu" "linux" "arm64" "so"

# Windows targets
build_target "x86_64-pc-windows-msvc" "windows" "x86_64" "dll"

echo ""
echo "Cross-platform build complete!"
echo ""
echo "For Apple Silicon users:"
echo "- Use the library from addons/gdserial/bin/macos-arm64/"
echo "- Ensure Godot 4.2+ is being used for proper ARM64 support"
echo ""
echo "Directory structure:"
find addons/gdserial/bin -name "*.so" -o -name "*.dylib" -o -name "*.dll" 2>/dev/null | sort