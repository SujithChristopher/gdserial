# GdSerial Example Project

This is a simple Godot 4 example project for testing the GdSerial plugin with thread-safe serial communication.

## Features

✓ **Thread-Safe Serial Communication** (v0.3.0+)
- GdSerial uses interior mutability with `Arc<Mutex<>>`
- All methods are safe to call from background threads
- All Godot RefCounted operations happen safely

✓ **Simple UI for Testing**
- Open/Close port
- Send data
- Read data continuously
- Clear display

## Setup

### Automatic (After Build)

1. Build the GdSerial Rust extension:
   ```bash
   # Windows
   .\build_release.bat

   # Linux/macOS
   ./build_release.sh
   ```

2. The addon will be **automatically copied** to `example/addons/`

3. Open this project in Godot 4.x

### Manual

If you've built GdSerial separately:

1. Copy the `addons/gdserial/` folder to `example/addons/gdserial/`
2. Open this project in Godot 4.x
3. Rebuild the project

## How to Test

1. **Connect a serial device** (Arduino, USB serial adapter, etc.)

2. **Open Godot** and load this example project

3. **Update the port name** in the "Port:" field:
   - Windows: `COM3`, `COM4`, etc.
   - Linux: `/dev/ttyUSB0`, `/dev/ttyACM0`, etc.
   - macOS: `/dev/tty.usbserial-*`, `/dev/tty.usbmodem*`, etc.

4. **Click "Open Port"** to connect

5. **Test communication:**
   - Type text in "Send:" field and click "Send"
   - Click "Read Once" to read one line
   - Data appears in the right panel automatically

6. **Check console output** for detailed debug information

## Thread Safety Testing

The thread-safe implementation (v0.3.0+) means:

- ✓ Multiple threads can call GdSerial methods safely
- ✓ No crashes due to concurrent access
- ✓ Automatic synchronization with `Arc<Mutex<>>`
- ✓ Lock overhead: ~10-20ns (negligible for serial I/O)

See `CLAUDE.md` in the root project for technical details.

## Troubleshooting

### Port Not Found
- Check the console for available ports
- Make sure the device is connected
- Try a different port name

### No Data Received
- Check baud rate matches your device (default: 9600)
- Check the timeout setting (default: 100ms)
- Try sending test data from the device

### Data Garbled
- Check baud rate and serial settings
- Try lowering the timeout value
- Restart the device

## Example Code

See `main.gd` for a complete example of:
- Opening/closing serial ports
- Sending and receiving data
- Handling serial errors
- Thread-safe communication patterns
