use godot::prelude::*;
use serialport::{SerialPort, SerialPortType, DataBits, Parity, StopBits, FlowControl, ErrorKind};
use std::time::Duration;
use std::io::{self, Read};

/// Build human-readable USB device name from descriptors
/// Optimized to minimize allocations by building string directly
fn get_usb_device_name(vid: u16, pid: u16, manufacturer: &Option<String>, product: &Option<String>) -> String {
    // Extract trimmed manufacturer string if available
    let mfg = manufacturer.as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    // Extract trimmed product string if available
    let prod = product.as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    // Build name based on available descriptors
    match (mfg, prod) {
        (Some(m), Some(p)) => {
            // Both available: pre-allocate exact capacity needed
            let mut name = String::with_capacity(m.len() + 1 + p.len());
            name.push_str(m);
            name.push(' ');
            name.push_str(p);
            name
        }
        (Some(m), None) => m.to_string(),
        (None, Some(p)) => p.to_string(),
        (None, None) => {
            // No descriptors: show VID/PID for identification
            format!("USB Serial (VID: 0x{:04X}, PID: 0x{:04X})", vid, pid)
        }
    }
}

struct GdSerialExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GdSerialExtension {}

// Configuration constants
const DEFAULT_BAUD_RATE: u32 = 9600;
const DEFAULT_TIMEOUT_MS: u64 = 1000;
const READLINE_BUFFER_SIZE: usize = 256;
const READLINE_INITIAL_CAPACITY: usize = 64;

// Static strings for dictionary keys to avoid repeated allocations
const KEY_PORT_NAME: &str = "port_name";
const KEY_PORT_TYPE: &str = "port_type";
const KEY_DEVICE_NAME: &str = "device_name";

// Static strings for port types
const PORT_TYPE_PCI: &str = "PCI";
const PORT_TYPE_BLUETOOTH: &str = "Bluetooth";
const PORT_TYPE_UNKNOWN: &str = "Unknown";
const DEVICE_NAME_PCI: &str = "PCI Serial Port";
const DEVICE_NAME_BLUETOOTH: &str = "Bluetooth Serial Port";
const DEVICE_NAME_UNKNOWN: &str = "Unknown Serial Device";

// Common error messages
const ERR_PORT_NOT_OPEN: &str = "Port not open";

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct GdSerial {
    base: Base<RefCounted>,
    port: Option<Box<dyn SerialPort>>,
    port_name: String,
    baud_rate: u32,
    timeout: Duration,
}

#[godot_api]
impl IRefCounted for GdSerial {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            port: None,
            port_name: String::new(),
            baud_rate: DEFAULT_BAUD_RATE,
            timeout: Duration::from_millis(DEFAULT_TIMEOUT_MS),
        }
    }
}

#[godot_api]
impl GdSerial {
    /// Check if an IO error kind indicates device disconnection
    #[inline]
    fn is_disconnection_io_kind(kind: io::ErrorKind) -> bool {
        matches!(kind,
            io::ErrorKind::BrokenPipe |
            io::ErrorKind::ConnectionAborted |
            io::ErrorKind::NotConnected |
            io::ErrorKind::UnexpectedEof |
            io::ErrorKind::PermissionDenied  // Can occur on disconnect
        )
    }

    /// Check if a serialport error indicates disconnection
    #[inline]
    fn is_disconnection_error(error: &serialport::Error) -> bool {
        match error.kind() {
            ErrorKind::NoDevice => true,
            ErrorKind::Io(io_error) => Self::is_disconnection_io_kind(io_error),
            _ => false
        }
    }

    /// Handle potential disconnection by closing the port if device is no longer available
    fn handle_potential_disconnection(&mut self, error: &serialport::Error) {
        if Self::is_disconnection_error(error) {
            godot_print!("Device disconnected, closing port");
            self.port = None;
        }
    }

    /// Handle potential disconnection for IO errors
    #[inline]
    fn handle_potential_io_disconnection(&mut self, error: &io::Error) {
        if Self::is_disconnection_io_kind(error.kind()) {
            godot_print!("Device disconnected (IO error), closing port");
            self.port = None;
        }
    }

    #[func]
    pub fn list_ports(&self) -> Dictionary {
        let mut ports_dict = Dictionary::new();

        match serialport::available_ports() {
            Ok(ports) => {
                for (i, port) in ports.iter().enumerate() {
                    let mut port_info = Dictionary::new();
                    port_info.set(KEY_PORT_NAME, port.port_name.as_str());

                    let (port_type, device_name) = match &port.port_type {
                        SerialPortType::UsbPort(usb_info) => {
                            let port_type = format!("USB - VID: {:04X}, PID: {:04X}",
                                   usb_info.vid, usb_info.pid);
                            let device_name = get_usb_device_name(
                                usb_info.vid,
                                usb_info.pid,
                                &usb_info.manufacturer,
                                &usb_info.product
                            );
                            (port_type, device_name)
                        }
                        SerialPortType::PciPort => (PORT_TYPE_PCI.to_string(), DEVICE_NAME_PCI.to_string()),
                        SerialPortType::BluetoothPort => (PORT_TYPE_BLUETOOTH.to_string(), DEVICE_NAME_BLUETOOTH.to_string()),
                        SerialPortType::Unknown => (PORT_TYPE_UNKNOWN.to_string(), DEVICE_NAME_UNKNOWN.to_string()),
                    };

                    port_info.set(KEY_PORT_TYPE, port_type.as_str());
                    port_info.set(KEY_DEVICE_NAME, device_name.as_str());
                    ports_dict.set(i as i32, port_info);
                }
            }
            Err(e) => {
                godot_error!("Failed to list ports: {}", e);
            }
        }

        ports_dict
    }
    
    #[func]
    #[inline]
    pub fn set_port(&mut self, port_name: GString) {
        self.port_name = port_name.to_string();
    }

    #[func]
    #[inline]
    pub fn set_baud_rate(&mut self, baud_rate: u32) {
        self.baud_rate = baud_rate;
    }

    #[func]
    #[inline]
    pub fn set_timeout(&mut self, timeout_ms: u32) {
        self.timeout = Duration::from_millis(timeout_ms as u64);
    }
    
    #[func]
    pub fn open(&mut self) -> bool {
        if self.port_name.is_empty() {
            godot_error!("Port name not set");
            return false;
        }
        
        match serialport::new(&self.port_name, self.baud_rate)
            .timeout(self.timeout)
            .data_bits(DataBits::Eight)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .flow_control(FlowControl::None)
            .open()
        {
            Ok(port) => {
                self.port = Some(port);
                true
            }
            Err(e) => {
                godot_error!("Failed to open port {}: {}", self.port_name, e);
                false
            }
        }
    }
    
    #[func]
    #[inline]
    pub fn close(&mut self) {
        if self.port.is_some() {
            self.port = None;
            // Port closed - removed print output per issue #1
        }
    }

    #[func]
    #[inline]
    pub fn is_open(&self) -> bool {
        // Simply check if port exists - don't make extra system calls
        self.port.is_some()
    }
    
    #[func]
    pub fn write(&mut self, data: PackedByteArray) -> bool {
        match &mut self.port {
            Some(port) => {
                let bytes = data.to_vec();
                match port.write_all(&bytes) {
                    Ok(_) => {
                        match port.flush() {
                            Ok(_) => true,
                            Err(e) => {
                                self.handle_potential_io_disconnection(&e);
                                godot_error!("Failed to flush port: {}", e);
                                false
                            }
                        }
                    }
                    Err(e) => {
                        self.handle_potential_io_disconnection(&e);
                        godot_error!("Failed to write to port: {}", e);
                        false
                    }
                }
            }
            None => {
                godot_error!("{}", ERR_PORT_NOT_OPEN);
                false
            }
        }
    }

    #[func]
    pub fn write_string(&mut self, data: GString) -> bool {
        // Avoid intermediate String allocation by converting directly to bytes
        let bytes = data.to_string();
        let packed_bytes = PackedByteArray::from(bytes.as_bytes());
        self.write(packed_bytes)
    }

    #[func]
    pub fn writeline(&mut self, data: GString) -> bool {
        // Avoid allocations by reusing write logic with static newline
        let string = data.to_string();
        let mut bytes = Vec::with_capacity(string.len() + 1);
        bytes.extend_from_slice(string.as_bytes());
        bytes.push(b'\n');
        let packed_bytes = PackedByteArray::from(&bytes[..]);
        self.write(packed_bytes)
    }
    
    #[func]
    pub fn read(&mut self, size: u32) -> PackedByteArray {
        match &mut self.port {
            Some(port) => {
                let mut buffer = vec![0; size as usize];
                match port.read(&mut buffer) {
                    Ok(bytes_read) => {
                        buffer.truncate(bytes_read);
                        PackedByteArray::from(&buffer[..])
                    }
                    Err(e) => {
                        // Don't treat timeout as disconnection
                        if e.kind() != io::ErrorKind::TimedOut && e.kind() != io::ErrorKind::WouldBlock {
                            self.handle_potential_io_disconnection(&e);
                            godot_error!("Failed to read from port: {}", e);
                        }
                        PackedByteArray::new()
                    }
                }
            }
            None => {
                godot_error!("{}", ERR_PORT_NOT_OPEN);
                PackedByteArray::new()
            }
        }
    }

    #[func]
    #[inline]
    pub fn read_string(&mut self, size: u32) -> GString {
        let bytes = self.read(size);
        // Use from_utf8_lossy to avoid allocation + handle invalid UTF-8 gracefully
        // This is more efficient than to_vec() + from_utf8()
        if bytes.is_empty() {
            return GString::new();
        }

        // Convert to slice and then to GString (avoids intermediate copy)
        let slice = bytes.as_slice();
        match std::str::from_utf8(slice) {
            Ok(s) => GString::from(s),
            Err(e) => {
                godot_error!("Failed to convert bytes to string: {}", e);
                GString::new()
            }
        }
    }
    
    #[func]
    pub fn readline(&mut self) -> GString {
        match &mut self.port {
            Some(port) => {
                // Use a buffer for more efficient reading (reduces system calls significantly)
                let mut line = String::with_capacity(READLINE_INITIAL_CAPACITY);
                let mut buffer = [0u8; READLINE_BUFFER_SIZE];
                let mut buffer_pos = 0;
                let mut buffer_len = 0;

                loop {
                    // Refill buffer if empty
                    if buffer_pos >= buffer_len {
                        match port.read(&mut buffer) {
                            Ok(0) => break, // No more data
                            Ok(n) => {
                                buffer_len = n;
                                buffer_pos = 0;
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => break,
                            Err(e) => {
                                if Self::is_disconnection_io_kind(e.kind()) {
                                    self.handle_potential_io_disconnection(&e);
                                }

                                if line.is_empty() && e.kind() != io::ErrorKind::WouldBlock {
                                    godot_error!("Failed to read line: {}", e);
                                    return GString::new();
                                } else {
                                    break;
                                }
                            }
                        }
                    }

                    // Process buffered data
                    while buffer_pos < buffer_len {
                        let ch = buffer[buffer_pos] as char;
                        buffer_pos += 1;

                        if ch == '\n' {
                            return GString::from(&line);
                        } else if ch != '\r' {
                            line.push(ch);
                        }
                    }
                }

                GString::from(&line)
            }
            None => {
                godot_error!("{}", ERR_PORT_NOT_OPEN);
                GString::new()
            }
        }
    }

    #[func]
    pub fn bytes_available(&mut self) -> u32 {
        // Don't call test_connection() here to avoid double-calling bytes_to_read()
        // Direct call is sufficient - bytes_to_read() itself tells us if port is working
        match &mut self.port {
            Some(port) => {
                match port.bytes_to_read() {
                    Ok(bytes) => bytes as u32,
                    Err(e) => {
                        // Only mark as disconnected for actual disconnection errors
                        if Self::is_disconnection_error(&e) {
                            godot_error!("Failed to get available bytes: {} - marking port as disconnected", e);
                            self.port = None;
                        }
                        0
                    }
                }
            }
            None => 0
        }
    }
    
    #[func]
    pub fn clear_buffer(&mut self) -> bool {
        match &mut self.port {
            Some(port) => {
                match port.clear(serialport::ClearBuffer::All) {
                    Ok(_) => true,
                    Err(e) => {
                        self.handle_potential_disconnection(&e);
                        godot_error!("Failed to clear buffer: {}", e);
                        false
                    }
                }
            }
            None => {
                godot_error!("{}", ERR_PORT_NOT_OPEN);
                false
            }
        }
    }
}