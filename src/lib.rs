use godot::prelude::*;
use serialport::{SerialPort, SerialPortType, DataBits, Parity, StopBits, FlowControl, ErrorKind};
use std::time::Duration;
use std::io::{self, Read};

fn get_usb_device_name(vid: u16, pid: u16, manufacturer: &Option<String>, product: &Option<String>) -> String {
    // Build device name from available USB descriptor information
    let mut parts = Vec::new();
    
    // Add manufacturer if available
    if let Some(mfg) = manufacturer {
        if !mfg.trim().is_empty() {
            parts.push(mfg.trim().to_string());
        }
    }
    
    // Add product if available
    if let Some(prod) = product {
        if !prod.trim().is_empty() {
            parts.push(prod.trim().to_string());
        }
    }
    
    // If we have any descriptor strings, use them
    if !parts.is_empty() {
        return parts.join(" ");
    }
    
    // Otherwise, show VID/PID for identification
    format!("USB Serial (VID: 0x{:04X}, PID: 0x{:04X})", vid, pid)
}

struct GdSerialExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GdSerialExtension {}

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct GdSerial {
    base: Base<RefCounted>,
    port: Option<Box<dyn SerialPort>>,
    port_name: String,
    baud_rate: u32,
    timeout: Duration,
    is_connected: bool,  // Track connection state
}

#[godot_api]
impl IRefCounted for GdSerial {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            port: None,
            port_name: String::new(),
            baud_rate: 9600,
            timeout: Duration::from_millis(1000),
            is_connected: false,
        }
    }
}

#[godot_api]
impl GdSerial {
    /// Check if the error indicates a disconnected device
    fn is_disconnection_error(error: &serialport::Error) -> bool {
        match error.kind() {
            ErrorKind::NoDevice => true,
            ErrorKind::Io(io_error) => {
                // Check for common disconnection errors
                matches!(io_error, 
                    io::ErrorKind::BrokenPipe | 
                    io::ErrorKind::ConnectionAborted |
                    io::ErrorKind::NotConnected |
                    io::ErrorKind::UnexpectedEof |
                    io::ErrorKind::PermissionDenied  // Can occur on disconnect
                )
            }
            _ => false
        }
    }
    
    /// Check if IO error indicates disconnection
    fn is_io_disconnection_error(error: &io::Error) -> bool {
        matches!(error.kind(), 
            io::ErrorKind::BrokenPipe | 
            io::ErrorKind::ConnectionAborted |
            io::ErrorKind::NotConnected |
            io::ErrorKind::UnexpectedEof |
            io::ErrorKind::PermissionDenied
        )
    }
    
    /// Handle potential disconnection by closing the port if device is no longer available
    fn handle_potential_disconnection(&mut self, error: &serialport::Error) {
        if Self::is_disconnection_error(error) {
            godot_print!("Device disconnected, closing port");
            self.port = None;
            self.is_connected = false;
        }
    }
    
    /// Handle potential disconnection for IO errors
    fn handle_potential_io_disconnection(&mut self, error: &io::Error) {
        if Self::is_io_disconnection_error(error) {
            godot_print!("Device disconnected (IO error), closing port");
            self.port = None;
            self.is_connected = false;
        }
    }
    
    /// Actively test if the port is still connected by attempting a non-destructive operation
    fn test_connection(&mut self) -> bool {
        if let Some(ref mut port) = self.port {
            // Try bytes_to_read as the primary test - it's the most reliable
            match port.bytes_to_read() {
                Ok(_) => true,
                Err(e) => {
                    // Any error here likely means disconnection
                    godot_print!("Connection test failed: {} - marking as disconnected", e);
                    self.port = None;
                    self.is_connected = false;
                    false
                }
            }
        } else {
            false
        }
    }
    
    #[func]
    pub fn list_ports(&self) -> Dictionary {
        let mut ports_dict = Dictionary::new();
        
        match serialport::available_ports() {
            Ok(ports) => {
                for (i, port) in ports.iter().enumerate() {
                    let mut port_info = Dictionary::new();
                    port_info.set(GString::from("port_name"), GString::from(&port.port_name));
                    
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
                        SerialPortType::PciPort => ("PCI".to_string(), "PCI Serial Port".to_string()),
                        SerialPortType::BluetoothPort => ("Bluetooth".to_string(), "Bluetooth Serial Port".to_string()),
                        SerialPortType::Unknown => ("Unknown".to_string(), "Unknown Serial Device".to_string()),
                    };
                    
                    port_info.set(GString::from("port_type"), GString::from(port_type));
                    port_info.set(GString::from("device_name"), GString::from(device_name));
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
    pub fn set_port(&mut self, port_name: GString) {
        self.port_name = port_name.to_string();
    }
    
    #[func]
    pub fn set_baud_rate(&mut self, baud_rate: u32) {
        self.baud_rate = baud_rate;
    }
    
    #[func]
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
                self.is_connected = true;
                true
            }
            Err(e) => {
                godot_error!("Failed to open port {}: {}", self.port_name, e);
                self.is_connected = false;
                false
            }
        }
    }
    
    #[func]
    pub fn close(&mut self) {
        if self.port.is_some() {
            self.port = None;
            self.is_connected = false;
            // Port closed - removed print output per issue #1
        }
    }
    
    #[func]
    pub fn is_open(&mut self) -> bool {
        // Always test the actual connection state
        if self.port.is_some() {
            self.test_connection()
        } else {
            false
        }
    }
    
    #[func]
    pub fn write(&mut self, data: PackedByteArray) -> bool {
        // First check if connected
        if !self.test_connection() {
            godot_error!("Port not connected");
            return false;
        }
        
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
                godot_error!("Port not open");
                false
            }
        }
    }
    
    #[func]
    pub fn write_string(&mut self, data: GString) -> bool {
        let bytes = data.to_string().into_bytes();
        let packed_bytes = PackedByteArray::from(&bytes[..]);
        self.write(packed_bytes)
    }
    
    #[func]
    pub fn writeline(&mut self, data: GString) -> bool {
        let data_with_newline = format!("{}\n", data.to_string());
        let bytes = data_with_newline.into_bytes();
        let packed_bytes = PackedByteArray::from(&bytes[..]);
        self.write(packed_bytes)
    }
    
    #[func]
    pub fn read(&mut self, size: u32) -> PackedByteArray {
        // First check if connected
        if !self.test_connection() {
            return PackedByteArray::new();
        }
        
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
                godot_error!("Port not open");
                PackedByteArray::new()
            }
        }
    }
    
    #[func]
    pub fn read_string(&mut self, size: u32) -> GString {
        let bytes = self.read(size);
        match String::from_utf8(bytes.to_vec()) {
            Ok(string) => GString::from(string),
            Err(e) => {
                godot_error!("Failed to convert bytes to string: {}", e);
                GString::new()
            }
        }
    }
    
    #[func]
    pub fn readline(&mut self) -> GString {
        // First check if connected
        if !self.test_connection() {
            return GString::new();
        }
        
        match &mut self.port {
            Some(port) => {
                let mut line = String::new();
                let mut byte = [0u8; 1];
                
                loop {
                    match port.read(&mut byte) {
                        Ok(0) => {
                            // No data available, return what we have so far
                            break;
                        }
                        Ok(_) => {
                            let ch = byte[0] as char;
                            if ch == '\n' {
                                break;
                            } else if ch != '\r' {
                                line.push(ch);
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                            // Timeout occurred, return what we have so far
                            break;
                        }
                        Err(e) => {
                            if Self::is_io_disconnection_error(&e) {
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
                
                GString::from(line)
            }
            None => {
                godot_error!("Port not open");
                GString::new()
            }
        }
    }
    
    #[func]
    pub fn bytes_available(&mut self) -> u32 {
        // First check if connected
        if !self.test_connection() {
            return 0;
        }
        
        match &mut self.port {
            Some(port) => {
                match port.bytes_to_read() {
                    Ok(bytes) => bytes as u32,
                    Err(e) => {
                        // Any error in bytes_to_read likely means the port is in a bad state
                        // Mark as disconnected regardless of error type
                        godot_error!("Failed to get available bytes: {} - marking port as disconnected", e);
                        self.port = None;
                        self.is_connected = false;
                        0
                    }
                }
            }
            None => 0
        }
    }
    
    #[func]
    pub fn clear_buffer(&mut self) -> bool {
        // First check if connected
        if !self.test_connection() {
            return false;
        }
        
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
                godot_error!("Port not open");
                false
            }
        }
    }
}