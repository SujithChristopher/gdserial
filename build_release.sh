#!/bin/bash
echo "Building GdSerial for release..."

# Build the Rust library
cargo build --release

# Create bin directory if it doesn't exist
mkdir -p addons/gdserial/bin

# Copy the built library to the addon directory
cp target/release/libgdserial.so addons/gdserial/bin/ 2>/dev/null || true
cp target/release/libgdserial.dylib addons/gdserial/bin/ 2>/dev/null || true

echo "Build complete! Library files copied to addons/gdserial/bin/"
echo ""
echo "To publish to Godot Asset Library:"
echo "1. Test the addon in a Godot project"
echo "2. Create a release on GitHub with the addons/ folder"  
echo "3. Submit to Godot Asset Library with the release URL"