# GdSerial - Serial Communication Library for Godot 4

<img src="addons/gdserial/icon.png" alt="GdSerial Icon" width="64" height="64" align="left" style="margin-right: 20px;">

A high-performance Rust-based serial communication library for Godot 4 game engine, providing PySerial-like functionality through gdext. Enable direct hardware communication with Arduino, ESP32, sensors, modems, and IoT devices in your Godot games and applications.

<br clear="left">

## Features

- **Arduino & ESP32 Integration**: Direct communication with microcontrollers and development boards
- **Hardware Sensor Support**: Interface with temperature, motion, GPS, and environmental sensors
- **Cross-platform Serial Communication**: Works on Windows, Linux, and macOS
- **PySerial-like API**: Familiar interface for Python developers transitioning to Godot
- **Game Engine Integration**: Native Godot 4 types and error handling for seamless gamedev workflow
- **IoT Device Communication**: Connect with WiFi modules, Bluetooth adapters, and cellular modems
- **Real-time Data Streaming**: Low-latency binary and text operations for responsive applications
- **Port Auto-discovery**: Automatic enumeration and identification of available serial devices

## Platform Support

| Platform | Architecture | Status | Port Examples |
|----------|--------------|--------|---------------|
| **Windows** | x64 | ✅ Supported | `COM1`, `COM3`, `COM8` |
| **Linux** | x64 | ✅ Supported | `/dev/ttyUSB0`, `/dev/ttyACM0` |
| **Linux** | ARM64 | 📋 Planned | `/dev/ttyUSB0`, `/dev/ttyACM0` |
| **macOS** | x64 (Intel) | ✅ Supported | `/dev/tty.usbserial-*`, `/dev/tty.usbmodem*` |
| **macOS** | ARM64 (Apple Silicon) | ✅ Supported | `/dev/tty.usbserial-*`, `/dev/tty.usbmodem*` |

## Installation

### Option 1: GitHub Releases (Recommended)

1. Download the latest release for your platform from [GitHub Releases](https://github.com/SujithChristopher/gdserial/releases)
2. Extract the `addons/gdserial` folder to your project's `addons/` directory
3. Go to Project > Project Settings > Plugins
4. Enable the "GdSerial - Serial Communication Library" plugin

### Option 2: Build from Source

#### Prerequisites
- Rust (latest stable version)
- Godot 4.2+
- Git

#### Building Steps
1. Clone this repository:
```bash
git clone https://github.com/SujithChristopher/gdserial.git
cd gdserial
```

2. Build the library:
```bash
# Linux/Mac
./build_release.sh

# Windows
build_release.bat
```

3. The plugin will be ready in the `addons/gdserial` folder with compiled libraries.

## API Reference

### Core Methods

#### Port Management
- `list_ports() -> Dictionary` - Get all available serial ports
- `set_port(port_name: String)` - Set the port to use (e.g., "COM3", "/dev/ttyUSB0")
- `set_baud_rate(rate: int)` - Set baud rate (default: 9600)
- `set_timeout(timeout_ms: int)` - Set read timeout in milliseconds (default: 1000)
- `open() -> bool` - Open the serial port
- `close()` - Close the serial port
- `is_open() -> bool` - Check if port is open

#### Data Operations
- `write(data: PackedByteArray) -> bool` - Write raw bytes
- `write_string(data: String) -> bool` - Write string data
- `writeline(data: String) -> bool` - Write string with newline
- `read(size: int) -> PackedByteArray` - Read raw bytes
- `read_string(size: int) -> String` - Read and convert to string
- `readline() -> String` - Read until newline character

#### Utilities
- `bytes_available() -> int` - Get number of bytes waiting to be read
- `clear_buffer() -> bool` - Clear input/output buffers

## Quick Start

After installing the plugin, you can use GdSerial in any script:

```gdscript
extends Node

var serial: GdSerial

func _ready():
    # Create serial instance
    serial = GdSerial.new()
    
    # List available ports
    print("Available ports:")
    var ports = serial.list_ports()
    for i in range(ports.size()):
        var port_info = ports[i]
        print("- ", port_info["port_name"], " (", port_info["port_type"], ")")
    
    # Configure and open port
    serial.set_port("COM3")  # Adjust for your system
    serial.set_baud_rate(115200)
    serial.set_timeout(1000)
    
    if serial.open():
        print("Port opened successfully!")
        
        # Send command
        serial.writeline("Hello Arduino!")
        
        # Wait and read response
        await get_tree().create_timer(0.1).timeout
        if serial.bytes_available() > 0:
            var response = serial.readline()
            print("Response: ", response)
        
        serial.close()
    else:
        print("Failed to open port")
```

> **Note**: The GdSerial class becomes available automatically once the plugin is enabled. No imports needed!

## Common Use Cases

### Arduino Communication
```gdscript
# Send sensor reading request
serial.writeline("GET_SENSOR")
var reading = serial.readline()
print("Sensor value: ", reading)
```

### AT Commands (Modems, WiFi modules)
```gdscript
serial.writeline("AT+VERSION?")
var version = serial.readline()
print("Module version: ", version)
```

### Binary Data Transfer
```gdscript
var data = PackedByteArray([0x01, 0x02, 0x03, 0x04])
serial.write(data)
var response = serial.read(10)
```

## Platform-Specific Notes

### Windows
- Port names are typically `COM1`, `COM2`, etc.
- Administrator privileges may be required for some devices

### Linux
- Port names are typically `/dev/ttyUSB0`, `/dev/ttyACM0`, etc.
- User must be in the `dialout` group to access serial ports:
  ```bash
  sudo usermod -a -G dialout $USER
  ```

### macOS
- Port names are typically `/dev/tty.usbserial-*` or `/dev/tty.usbmodem*`

## Error Handling

The library provides comprehensive error logging through Godot's built-in logging system. Common errors include:

- Port not found or inaccessible
- Permission denied (check user permissions)
- Device disconnected during operation
- Timeout during read operations

Check the Godot console for detailed error messages when operations fail.

## Troubleshooting

### Port Not Found
- Verify the device is connected and recognized by the OS
- Check device manager (Windows) or `dmesg` (Linux)
- Try different USB ports or cables

### Permission Denied (Linux)
```bash
sudo usermod -a -G dialout $USER
# Log out and back in for changes to take effect
```

### Build Issues
- Ensure Rust is up to date: `rustup update`
- Clear cargo cache: `cargo clean`
- Check that all dependencies are available

## Plugin Structure

```
addons/gdserial/
├── plugin.cfg              # Plugin configuration
├── plugin.gd               # Plugin activation script
├── gdserial.gdextension    # Extension loader
├── bin/                    # Compiled libraries
│   ├── windows/            
│   │   └── gdserial.dll    # Windows x64
│   ├── linux/              
│   │   └── libgdserial.so  # Linux x64
│   └── macos/              
│       └── libgdserial.dylib # macOS x64
└── README.md              # Plugin documentation
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test with the build scripts
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Keywords & Topics

**GitHub Topics**: `godot`, `serial-communication`, `arduino`, `esp32`, `iot`, `gamedev`, `rust`, `pyserial`, `sensors`, `hardware`, `cross-platform`, `gdext`, `microcontroller`, `embedded`

**Search Keywords**: Godot serial port, Arduino Godot integration, ESP32 game development, IoT Godot plugin, hardware communication gamedev, PySerial Godot, sensor data Godot, real-time hardware control

## Dependencies

- [gdext](https://github.com/godot-rust/gdext) - Godot 4 Rust bindings
- [serialport](https://crates.io/crates/serialport) - Cross-platform serial port library