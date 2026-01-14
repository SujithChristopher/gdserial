# GdSerial Testing Guide

This guide explains how to build and test the thread-safe GdSerial plugin.

## Build & Test Workflow

### Step 1: Build the Plugin

**Windows:**
```bash
.\build_release.bat
```

**Linux/macOS:**
```bash
./build_release.sh
```

This will:
1. Compile the Rust extension (`cargo build --release`)
2. Copy the compiled library to `addons/gdserial/bin/[platform]/`
3. **Automatically copy the entire addon to `example/addons/gdserial/`**

### Step 2: Open Example Project

1. Navigate to the `example/` folder
2. Open in Godot 4.x (File → Open Project)
3. Run the project (F5)

### Step 3: Test

The example project provides a UI for testing:

- **Port Configuration:**
  - Set the port name (COM3, /dev/ttyUSB0, etc.)
  - Open/Close the port

- **Data Communication:**
  - Send text to the device
  - Receive data from the device
  - View communication log

- **Diagnostics:**
  - See bytes available
  - Check connection status
  - Monitor in console output

## What's New in v0.3.0

### Thread Safety

**The Problem (v0.2.8):**
- GdSerial could not be used from background threads
- Calling from threads caused crashes
- Issue #12 reported this

**The Solution (v0.3.0):**
- All state wrapped in `Arc<Mutex<>>`
- All methods changed from `&mut self` to `&self`
- Interior mutability pattern for thread-safe access
- No breaking changes for GDScript

### Architecture

```rust
pub struct GdSerial {
    base: Base<RefCounted>,
    state: Arc<Mutex<GdSerialState>>,  // Thread-safe wrapper
}
```

**Key benefits:**
- ✓ Safe concurrent access
- ✓ No data races
- ✓ Lock overhead: ~10-20ns (negligible for I/O)
- ✓ Backward compatible API

## Testing Scenarios

### Basic Functionality (v0.2.8 and v0.3.0)
1. Open a serial port
2. Send data to device
3. Receive data from device
4. Close port

**Run:** Click buttons in UI, check console output

### Thread Safety (v0.3.0 only)
1. Open serial port from main thread
2. Create background thread
3. Call GdSerial methods from thread
4. Should not crash

**Run:** Modify `example/main.gd` to spawn threads and call GdSerial methods

### Disconnection Handling
1. Open serial port
2. Unplug the device
3. Try to read/write
4. Should gracefully handle disconnection

**Run:** Open port, unplug device, try to send data

## Build Scripts

### copy_addon.bat (Windows)
Copies `addons/gdserial/` → `example/addons/gdserial/`
- Called automatically by `build_release.bat`
- Can be run manually: `copy_addon.bat`

### copy_addon.sh (Unix)
Same as above for Linux/macOS
- Called automatically by `build_release.sh`
- Can be run manually: `./copy_addon.sh`

## File Structure

```
gdserial/
├── src/
│   └── lib.rs                    # Rust source (thread-safe v0.3.0)
├── addons/gdserial/
│   ├── bin/                      # Compiled binaries by platform
│   ├── doc/                      # Godot documentation
│   └── gdserial.gdextension      # GDExtension config
├── example/                      # Example Godot project (auto-synced)
│   ├── addons/gdserial/          # Auto-copied from root addons/
│   ├── main.gd                   # Main script
│   ├── main.tscn                 # UI scene
│   └── project.godot             # Godot config
├── build_release.bat             # Build + copy (Windows)
├── build_release.sh              # Build + copy (Unix)
├── copy_addon.bat                # Manual copy script (Windows)
└── copy_addon.sh                 # Manual copy script (Unix)
```

## Continuous Integration

The build scripts are designed for CI/CD:

1. Run `./build_release.sh` (or `.bat` on Windows)
2. Build artifacts are in `target/release/`
3. Binary is copied to platform-specific directory
4. Addon is synced to example project
5. Example is ready to test

## Troubleshooting

### Build Fails
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### Example Project Not Updated
```bash
# Run copy scripts manually
# Windows:
copy_addon.bat

# Unix:
./copy_addon.sh
```

### Godot Can't Find Plugin
1. Check `example/addons/gdserial/` exists
2. Check `.gdextension` file is present
3. Reload project (File → Reload Project)
4. Rebuild C# project if needed

### Serial Port Issues
1. Check port name is correct
2. Check baud rate matches device
3. Check device drivers are installed
4. Check device is connected
5. Try different timeout value

## Documentation

- `src/lib.rs` - Implementation with inline comments
- `CLAUDE.md` - Architecture and threading details
- `README.md` - User documentation
- `example/README.md` - Example project guide
