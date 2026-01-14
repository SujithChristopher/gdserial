# GdSerial Example Project

Two minimal examples demonstrating the GdSerial serial communication plugin for Godot 4.

## Quick Start

### 1. Build & Setup

```bash
# Build the plugin (Windows)
.\\build_release.bat

# Build the plugin (Linux/macOS)
./build_release.sh
```

The addon is automatically copied to `example/addons/` during the build.

### 2. Open in Godot

- Navigate to the `example/` folder
- Open in Godot 4.x
- Run the project (F5 or Play)

### 3. Test Communication

1. Connect a serial device (Arduino, USB adapter, etc.)
2. Set the port name (COM3, /dev/ttyUSB0, etc.)
3. Click "Open Port" to connect
4. Send and receive data

## Example 1: Simple Usage (simple_example.tscn)

Basic serial communication demonstration:

**Features:**
- Open/close serial port
- Send messages
- Read incoming data
- Real-time status display

**Code:** `simple_example.gd` (~50 lines)

**How to use:**
1. Run the scene (F5)
2. Enter port name (e.g., "COM3")
3. Click "Open Port"
4. Type a message and click "Send"
5. Click "Read Data" to receive messages

## Example 2: Thread Safety (thread_example.tscn)

Demonstrates GdSerial's thread-safe architecture (v0.3.0+):

**Features:**
- Background thread calls GdSerial methods
- Main thread remains responsive
- No crashes or data races
- Lock overhead: ~10-20ns (negligible)

**Code:** `thread_example.gd` (~35 lines)

**How to use:**
1. Connect a serial device on COM3
2. Run the scene (F5)
3. Click "Start Background Thread Test"
4. Watch success message confirm thread-safe operation

## Architecture (v0.3.0)

### What's Thread-Safe

✓ All read/write operations
✓ Configuration (baud rate, timeout, etc.)
✓ Port open/close
✓ Simultaneous access from multiple threads

### Interior Mutability Pattern

```rust
pub struct GdSerial {
    state: Arc<Mutex<GdSerialState>>  // Thread-safe wrapper
}

// All methods use &self (not &mut self)
pub fn read(&self, size: u32) -> PackedByteArray
pub fn write(&self, data: PackedByteArray) -> bool
```

### When to Use Threads

**Simple case (95% of applications):**
```gdscript
# Keep serial I/O in main thread
func _process(delta):
    if serial.bytes_available() > 0:
        var data = serial.readline()
        label.text = data  # GUI updates in main thread
```

**Advanced case (heavy computation):**
```gdscript
# Use threads for expensive work, main thread for I/O
func _thread_func():
    expensive_result = calculate_something()
    call_deferred("_on_complete", expensive_result)

func _process(delta):
    if serial.bytes_available() > 0:
        var data = serial.readline()  # Main thread handles serial I/O
```

## Troubleshooting

### Port Not Found
- Check device is connected
- Windows: Try COM1-COM10
- Linux: Check `/dev/ttyUSB*` or `/dev/ttyACM*`
- macOS: Check `/dev/tty.usbserial-*` or `/dev/tty.usbmodem*`

### No Data Received
- Verify baud rate matches device (default: 9600)
- Check timeout setting (default: 100ms)
- Verify device is sending data

### Data Garbled or Incomplete
- Check baud rate and serial settings match
- Verify no other application is using the port
- Try restarting the device

### Permission Denied (Linux)
```bash
# Add user to dialout group
sudo usermod -a -G dialout $USER
# Log out and log in again
```

## File Structure

```
example/
├── simple_example.gd       # Simple usage script (~50 lines)
├── simple_example.tscn     # Simple usage scene
├── thread_example.gd       # Thread safety demo (~35 lines)
├── thread_example.tscn     # Thread safety scene
├── addons/gdserial/        # Plugin (auto-copied during build)
├── project.godot           # Godot project configuration
└── README.md              # This file
```

## Documentation

- **Architecture Details:** See `CLAUDE.md` in the root project
- **Full API Reference:** See Godot's class reference (built-in after rebuild)
- **Threading Best Practices:** See threading examples in the example scripts

## Version

- Plugin: v0.3.0 (thread-safe with Arc<Mutex<>>)
- Godot: 4.5+
- Rust: 1.70+
