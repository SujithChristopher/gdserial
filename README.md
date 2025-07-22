# GdSerial

A Rust-based serial communication library for Godot 4, providing PySerial-like functionality through gdext. This library allows you to easily communicate with serial devices (Arduino, sensors, etc.) directly from your Godot projects.

## Features

- **Port Discovery**: Enumerate and identify available COM ports
- **Cross-platform**: Works on Windows, Linux, and macOS
- **PySerial-like API**: Familiar interface for Python developers
- **Godot Integration**: Native Godot types and error handling
- **Comprehensive I/O**: Support for both binary and text operations
- **Line-based Communication**: Built-in readline/writeline functionality

## Installation

### Prerequisites

- Rust (latest stable version)
- Godot 4.x
- Git

### Building the Extension

1. Clone this repository:
```bash
git clone <your-repo-url>
cd gdserial
```

2. Build the library:
```bash
cargo build --release
```

3. The compiled library will be in `target/release/`:
   - Windows: `gdserial.dll`
   - Linux: `libgdserial.so`
   - macOS: `libgdserial.dylib`

4. Copy the library to your Godot project's extension directory and create the necessary `.gdextension` file.

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

## Usage Example

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

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
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