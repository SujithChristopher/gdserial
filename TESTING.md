# Testing Guide for GdSerial

This document explains how to test the gdserial library without needing Godot.

## The Problem (Resolved)

**Issue**: `bytes_available()` was returning 0 even when data was available from the serial device.

**Root Cause**:
- Serialport crate versions 4.8.0+ introduced a bug on Windows where `bytes_to_read()` always returns 0
- Version 4.8.0 added advisory file locking with `flock` which broke the functionality
- Additionally, the original code was calling `bytes_to_read()` twice (once in `test_connection()` and once in `bytes_available()`) which could cause issues with the new locking mechanism

**Solution**:
1. Downgraded to `serialport = "4.7.2"` (last known working version)
2. Fixed `bytes_available()` to avoid double-calling `bytes_to_read()`
3. Pinned the exact serialport version in Cargo.lock to prevent automatic updates

## Standalone Test Programs

We've created test programs in the `examples/` directory that can test serial communication without Godot:

### 1. bytes_to_read_test.rs
Tests the `bytes_to_read()` function specifically - this is what `bytes_available()` uses internally.

**Usage:**
```bash
cargo run --example bytes_to_read_test COM10
```

**What it does:**
- Opens the specified serial port
- Calls `bytes_to_read()` 20 times with 500ms intervals
- Shows how many bytes are available each time
- Attempts to read and display data if bytes are available

### 2. arduino_monitor.rs
Continuous serial monitor similar to Arduino IDE's serial monitor.

**Usage:**
```bash
cargo run --example arduino_monitor COM10
```

**What it does:**
- Opens the serial port at 9600 baud
- Continuously monitors for incoming data
- Displays lines as they arrive
- Shows bytes_available count when data is detected
- Perfect for testing with Arduino sending periodic messages

### 3. test_serial.rs
General purpose serial port testing tool.

**Usage:**
```bash
cargo run --example test_serial
```

**What it does:**
- Lists all available serial ports
- Allows selection of a port to test
- Runs multiple tests: connection, bytes_to_read, write, read, clear buffer

## Example Arduino Code

To test, upload this simple sketch to your Arduino:

```cpp
void setup() {
  Serial.begin(9600);
}

void loop() {
  Serial.println("Hello from Arduino");
  delay(100);  // Send 10 times per second
}
```

## Expected Results

### With serialport 4.7.2 (WORKING):
```
Call 1: bytes_to_read() = 0
Call 2: bytes_to_read() = 12300
       Read 12300 bytes: "Hello from Arduino\r\nHello from Arduino\r\n..."
Call 3: bytes_to_read() = 24578
       Read 24578 bytes: "Hello from Arduino\r\nHello from Arduino\r\n..."
```

### With serialport 4.8.1 (BROKEN):
```
Call 1: bytes_to_read() = 0
Call 2: bytes_to_read() = 0
Call 3: bytes_to_read() = 0
...
```

## Version Information

- **Working version**: serialport 4.7.2
- **Broken versions**: serialport 4.8.0, 4.8.1
- **Breaking change introduced in 4.8.0**: Advisory file locking with `flock`

## How to Build

```bash
# Build the library
cargo build --release

# Build all examples
cargo build --examples

# Build specific example
cargo build --example arduino_monitor
```

## Files Modified

1. **Cargo.toml**: Changed serialport dependency from "4.8.1" to "=4.7.2" (exact version pin)
2. **src/lib.rs**:
   - Fixed `bytes_available()` method to avoid double-calling `bytes_to_read()`
   - Removed `test_connection()` function entirely (was causing unnecessary system calls)
   - Refactored all methods (`write()`, `read()`, `readline()`, `clear_buffer()`) to not use `test_connection()`
   - Changed `is_open()` to simply check if port exists (no system calls)
3. **Cargo.lock**: Locked to serialport 4.7.2 dependencies

## Performance Improvements

The refactoring also improved performance by eliminating redundant system calls:

**Before**: Each operation made 2 calls to the OS:
1. `test_connection()` calls `bytes_to_read()` to check if port is alive
2. The actual operation (read/write/clear)

**After**: Each operation makes only 1 call:
1. Try the operation directly - if it fails, handle the error

This reduces system call overhead by 50% for all serial operations!

## Important Notes

- The `=4.7.2` syntax in Cargo.toml is misleading - Cargo still tries to upgrade to 4.8.x
- The actual version control is through the committed Cargo.lock file
- DO NOT delete Cargo.lock - it's the only way to ensure 4.7.2 is used
- Future updates: Monitor serialport releases for fixes to the 4.8.x branch

## Testing Checklist

Before releasing a new version:

- [ ] Run `cargo run --example bytes_to_read_test COM10` (replace COM10 with your port)
- [ ] Verify bytes_to_read() returns non-zero values when data is available
- [ ] Test in Godot with `bytes_available()` method
- [ ] Verify data can be read successfully
- [ ] Test disconnection handling (unplug device while reading)
