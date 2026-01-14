# Quick Start - GdSerial v0.3.0 Testing

## In 3 Steps

### 1️⃣ Build the Plugin
```bash
# Windows
.\build_release.bat

# Linux/macOS
./build_release.sh
```

### 2️⃣ Open Example Project
- Navigate to `example/` folder
- Open in Godot 4.x
- Project is ready to test (addon auto-copied!)

### 3️⃣ Test Serial Communication
1. Connect a serial device
2. Set port (COM3, /dev/ttyUSB0, etc.)
3. Click "Open Port"
4. Send/receive data

## What's New (v0.3.0)

✅ **Thread-Safe** - All methods use interior mutability (`Arc<Mutex<>>`)
✅ **No Crashes** - Safe concurrent access from threads
✅ **Auto-Sync** - Addon auto-copied to example after build
✅ **Same API** - No breaking changes for GDScript

## New Files

```
example/                          # New Godot example project
├── main.gd                        # Interactive serial test UI
├── main.tscn                      # UI with buttons and display
├── project.godot                  # Godot 4 config
└── README.md                      # Example documentation

copy_addon.bat/sh                  # Auto-copy addon to example
TESTING.md                         # Detailed testing guide
QUICK_START.md                     # This file
```

## Architecture (v0.3.0)

**Before (v0.2.8):**
```rust
pub fn write(&mut self, data: PackedByteArray) -> bool
// ❌ Can't be called from multiple threads
```

**After (v0.3.0):**
```rust
pub struct GdSerial {
    state: Arc<Mutex<GdSerialState>>  // Thread-safe wrapper
}

pub fn write(&self, data: PackedByteArray) -> bool
// ✅ Can be called from any thread safely
```

## Test Checklist

- [ ] Build completes successfully
- [ ] Addon copied to `example/addons/`
- [ ] Example project opens in Godot
- [ ] Can open serial port
- [ ] Can send data
- [ ] Can receive data
- [ ] Can close port gracefully
- [ ] Console shows no errors

## Need Help?

- See `TESTING.md` for detailed guide
- See `example/README.md` for example-specific info
- Check `CLAUDE.md` for architecture details
- Check console output for errors

---

**Ready to test?** Run the build script and open the example project! 🚀
