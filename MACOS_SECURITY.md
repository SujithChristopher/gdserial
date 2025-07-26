# macOS Security & Code Signing

## User Installation Issues

If you get "malware" warnings or library loading errors on macOS, this is due to Apple's security system blocking unsigned binaries.

### Quick Fix (Recommended)

```bash
# Remove quarantine flag from the downloaded library
cd /path/to/your/godot/project/addons/gdserial/bin/macos-arm64/
xattr -d com.apple.quarantine libgdserial.dylib

# Or remove from entire addon folder
xattr -rd com.apple.quarantine /path/to/addons/gdserial/
```

### Alternative Methods

**Option 1: System Preferences**
1. Go to System Preferences → Security & Privacy → General
2. When blocked, click "Allow Anyway" for gdserial library

**Option 2: Command Line Override**
```bash
# Temporarily disable Gatekeeper (NOT recommended for security)
sudo spctl --master-disable

# Re-enable after testing
sudo spctl --master-enable
```

## Why This Happens

- **Gatekeeper**: Blocks unsigned apps/libraries from unknown developers
- **XProtect**: Scans downloads for malware signatures
- **Quarantine**: Flags internet-downloaded files for additional security checks

Dynamic libraries (`.dylib`) are treated more strictly than applications because they can be loaded by any process. Currently, I do not have an Apple Developer Program membership for this. Thank you for your understanding.