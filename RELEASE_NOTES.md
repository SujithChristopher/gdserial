# GdSerial v0.1.0 - Initial Release

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