# GdSerial

<img src="addons/gdserial/icon.png" alt="GdSerial Icon" width="64" height="64" align="left" style="margin-right: 20px;">

A Rust-based serial communication library for Godot 4, providing PySerial-like functionality through gdext. This library allows you to easily communicate with serial devices (Arduino, sensors, etc.) directly from your Godot projects.

<br clear="left">

## Features

- **Port Discovery**: Enumerate and identify available COM ports
- **Cross-platform**: Works on Windows, Linux, and macOS
- **PySerial-like API**: Familiar interface for Python developers
- **Godot Integration**: Native Godot types and error handling
- **Comprehensive I/O**: Support for both binary and text operations
- **Line-based Communication**: Built-in readline/writeline functionality

## Installation

### Option 1: Godot Asset Library (Recommended)

1. Open your Godot project
2. Go to Project > Project Settings > Plugins
3. Search for "GdSerial" in the Asset Library
4. Install and enable the plugin

### Option 2: Manual Installation

1. Download the latest release from GitHub
2. Extract the `addons/gdserial` folder to your project's `addons/` directory
3. Go to Project > Project Settings > Plugins
4. Enable the "GdSerial - Serial Communication Library" plugin

### Option 3: Build from Source

#### Prerequisites
- Rust (latest stable version)
- Godot 4.2+
- Git

#### Building Steps
1. Clone this repository:
```bash
git clone <your-repo-url>
cd gdserial
```

2. Build the library:
```bash
# Linux/Mac
./build_release.sh

# Windows
build_release.bat
```

3. The plugin will be ready in the `addons/gdserial` folder with compiled libraries for all platforms.

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

## Publishing and Distribution

### For Developers

To build and distribute this plugin:

1. **Build for release**:
```bash
./build_release.sh    # Linux/Mac
build_release.bat     # Windows
```

2. **Test locally**: Copy `addons/gdserial` to a test project and verify functionality

3. **Create release**: Package the `addons` folder for distribution

### Plugin Structure

```
addons/gdserial/
├── plugin.cfg              # Plugin configuration
├── plugin.gd               # Plugin activation script
├── gdserial.gdextension    # Extension loader
├── bin/                    # Compiled libraries
│   ├── gdserial.dll       # Windows
│   ├── libgdserial.so     # Linux
│   └── libgdserial.dylib  # macOS
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

## Dependencies

- [gdext](https://github.com/godot-rust/gdext) - Godot 4 Rust bindings
- [serialport](https://crates.io/crates/serialport) - Cross-platform serial port library

## Changelog

### v0.1.0
- Initial release
- Basic serial communication functionality
- Port enumeration and management
- String and binary I/O operations
- Cross-platform support
- Godot Asset Library ready plugin structure