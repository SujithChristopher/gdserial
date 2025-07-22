# GdSerial - Serial Communication for Godot

<img src="icon.png" alt="GdSerial Icon" width="64" height="64" align="left" style="margin-right: 20px;">

A Rust-based serial communication library for Godot 4 that provides PySerial-like functionality.

<br clear="left">

## Installation

1. Download this addon from the Godot Asset Library or GitHub
2. Copy the `addons/gdserial` folder to your project's `addons/` directory
3. Enable the plugin in Project Settings > Plugins

## Quick Start

```gdscript
extends Node

var serial: GdSerial

func _ready():
    serial = GdSerial.new()
    
    # List available ports
    var ports = serial.list_ports()
    print("Available ports: ", ports)
    
    # Configure and connect
    serial.set_port("COM3")  # Adjust for your system
    serial.set_baud_rate(9600)
    
    if serial.open():
        serial.writeline("Hello Arduino!")
        var response = serial.readline()
        print("Response: ", response)
        serial.close()
```

## API Reference

See the main repository README for complete API documentation.

## Requirements

- Godot 4.2+
- Appropriate permissions for serial port access (see platform-specific notes in main README)

## License

MIT License - see LICENSE file for details.