# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

GdSerial is a Rust-based serial communication library for Godot 4 that provides PySerial-like functionality through gdext. It enables direct communication with serial devices (Arduino, sensors, modems) from Godot projects.

## Build Commands

```bash
# Check for compilation errors (fastest feedback)
cargo check

# Build for development
cargo build

# Build optimized release (auto-detects platform/arch, copies to addons/)
./build_release.sh

# Build release binary manually (Windows)
cargo build --release
# Then copy: target/release/gdserial.dll → addons/gdserial/bin/windows-x86_64/
```

No test suite exists — testing is manual via `example.gd`.

## Dependencies

- `godot = "0.4.x"` (crates.io) — gdext framework for Godot 4 bindings
- `serialport = "4.7.x"` — cross-platform serial hardware interface
- No `parking_lot` — uses `std::sync::Mutex` directly

## Code Architecture

All code lives in `src/lib.rs`. There are two public Godot classes:

### `GdSerial` (single-port, synchronous)
Wraps one serial port with blocking read/write operations. State is held directly on the struct in an `Arc<Mutex<Box<dyn SerialPort>>>`. All public methods use `&self`/`&mut self` with internal locking. Intended for simple single-device use from the main thread or a Godot `Thread`.

### `GdSerialManager` (multi-port, async/signal-based)
Manages multiple ports simultaneously. Each port gets its own background reader thread that sends `ReaderEvent` messages over an `mpsc::channel` to the main thread. Signals (`data_received`, `port_disconnected`) are emitted when `poll_events()` is called from `_process()`.

**Typical GDScript usage:**
```gdscript
func _process(_delta):
    for event in serial_manager.poll_events():
        if event.get("disconnected"):
            print("Lost: ", event["port"])
        else:
            print("Data from ", event["port"], ": ", event["data"])
```

**Buffering modes** (exposed as class constants `MODE_RAW=0`, `MODE_LINE_BUFFERED=1`, `MODE_CUSTOM_DELIMITER=2`):
- `open(port, baud, timeout)` — opens with RAW mode (default)
- `open_buffered(port, baud, timeout, mode)` — opens with explicit mode
- `set_delimiter(port, byte)` — changes delimiter at runtime without reopening
- `reconfigure_port(port, baud, timeout_ms, data_bits, parity, stop_bits, flow_control)` — change serial params on an already-open port without reopening

**`GdSerial` uses a setter-then-open pattern** (`set_port()`, `set_baud_rate()`, `set_data_bits()`, etc.) rather than inline args. `GdSerialManager` takes all args directly in `open()`/`open_buffered()`.

### Disconnection Detection
Both classes detect device unplugging. `GdSerial` has `test_connection()` called before every I/O operation (uses `bytes_to_read()` as a non-destructive probe). `GdSerialManager` detects errors in the reader thread and sends a `ReaderEvent::Disconnected` message. Helper functions `is_disconnection_error()` and `is_io_disconnection_error()` classify `serialport::Error` vs `std::io::Error` types.

**Signal emission:** `port_disconnected` is always emitted by `close()` — both for manual closes and hardware disconnections (via `poll_events` → `close`). `poll_events` does not emit signals directly; it calls `close()` which is the single source of truth.

## Documentation

Godot class reference XML lives in `addons/gdserial/doc/en/`. When adding or changing any `#[func]` method, update the corresponding XML — it appears in Godot's built-in help.

## Version Management

Update version in **both** of these files when releasing:
- `Cargo.toml` — `version` field
- `addons/gdserial/plugin.cfg` — `version` field

## Platform Binary Paths

| Platform | Path |
|---|---|
| Windows x86_64 | `addons/gdserial/bin/windows-x86_64/gdserial.dll` |
| Linux x86_64 | `addons/gdserial/bin/linux-x86_64/libgdserial.so` |
| Linux ARM64 | `addons/gdserial/bin/linux-arm64/libgdserial.so` |
| macOS Intel | `addons/gdserial/bin/macos-x86_64/libgdserial.dylib` |
| macOS Apple Silicon | `addons/gdserial/bin/macos-arm64/libgdserial.dylib` |
