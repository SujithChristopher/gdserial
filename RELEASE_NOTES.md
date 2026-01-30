# GdSerial Release Notes

## GdSerial v0.1.0 - Initial Release

## 🚀 Features

- **Cross-platform serial communication** for Godot 4.2+
- **PySerial-like API** for familiar Python developers
- **Port enumeration** with device type detection (USB, PCI, Bluetooth)
- **Comprehensive I/O operations**: read, write, readline, writeline
- **Built-in error handling** with Godot logging integration
- **Timeout support** for reliable communication
- **Buffer management** utilities

## 📦 Installation

### From Godot Asset Library

1. Open Project Settings > Plugins
2. Search for "GdSerial"
3. Install and enable the plugin

### Manual Installation

1. Download the release archive
2. Extract `addons/gdserial` to your project's `addons/` directory
3. Enable the plugin in Project Settings

## 🔧 Supported Platforms

- **Windows**: COM ports (COM1, COM2, etc.)
- **Linux**: /dev/ttyUSB*, /dev/ttyACM* (requires dialout group)
- **macOS**: /dev/tty.usbserial-*, /dev/tty.usbmodem*

## 📚 Quick Start

```gdscript
var serial = GdSerial.new()
serial.set_port("COM3")
serial.set_baud_rate(9600)

if serial.open():
    serial.writeline("Hello Arduino!")
    var response = serial.readline()
    print("Response: ", response)
    serial.close()
```

## 🛠️ Technical Details

- Built with Rust and gdext for optimal performance
- Uses the `serialport` crate for cross-platform compatibility
- Compiled as dynamic library (cdylib) for Godot extension loading

## 📋 Requirements

- Godot 4.2+
- Appropriate system permissions for serial port access

## 🐛 Known Issues

None reported for initial release.

## 🔗 Links

- [GitHub Repository](https://github.com/your-username/gdserial)
- [Documentation](https://github.com/your-username/gdserial#readme)
- [Issues & Support](https://github.com/your-username/gdserial/issues)

---

# GdSerial v0.3.0 - Async Multi-Port Support

## 🚀 New Features

- **GdSerialManager**: New class for managing multiple serial ports simultaneously.
- **Asynchronous Reading**: Background threads handle data reception without blocking the Godot main thread.
- **Signal-based I/O**: `data_received` and `port_disconnected` signals for event-driven serial communication.
- **Improved Reliability**: Enhanced disconnection detection and connection state testing.
- **Advanced Configuration**: Full support for data bits, stop bits, parity, and flow control.

## 🛠️ Internal Changes

- Updated to latest Rust `serialport` crate.
- Improved error handling and Godot logging integration.
- Refactored disconnection handling for better robustness across platforms.
- **API Change**: `GdSerial.set_parity` now accepts an integer (0: None, 1: Odd, 2: Even) for consistency and more options.
