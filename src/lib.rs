use godot::prelude::*;
use serialport::{SerialPort, SerialPortType};
use std::time::Duration;

/// Get device name for a USB device based on USB descriptors and VID
fn get_usb_device_name(vid: u16, pid: u16, manufacturer: &Option<String>, product: &Option<String>) -> String {
    // Priority 1: Use the product string from USB descriptor if available
    if let Some(product) = product {
        // Many devices have good product names like "Arduino Uno R3", "ESP32-S3", etc.
        return product.clone();
    }
    
    // Priority 2: Use manufacturer + basic identification
    if let Some(manufacturer) = manufacturer {
        // Try to add some context based on well-known VIDs
        let device_type = match vid {
            0x2341 => "Arduino Board",           // Arduino
            0x16c0 => "Teensy Board",            // PJRC (Teensy)
            0x303a => "ESP32 Board",             // Espressif ESP32
            0x2e8a => "Raspberry Pi Board",      // Raspberry Pi Foundation
            0x239a => "Adafruit Board",          // Adafruit
            0x1b4f => "SparkFun Board",          // SparkFun
            0x0403 => "FTDI USB-Serial",         // FTDI
            0x10c4 => "Silicon Labs USB-Serial", // Silicon Labs
            0x1a86 => "USB-Serial Adapter",      // QinHeng (CH340/CH341)
            0x067b => "Prolific USB-Serial",     // Prolific
            _ => "USB Device"
        };
        return format!("{} {}", manufacturer, device_type);
    }
    
    // Priority 3: Generic names based on well-known VIDs
    match vid {
        0x2341 => format!("Arduino Board (VID: {:04X}, PID: {:04X})", vid, pid),
        0x16c0 => format!("Teensy Board (VID: {:04X}, PID: {:04X})", vid, pid),
        0x303a => format!("ESP32 Board (VID: {:04X}, PID: {:04X})", vid, pid),
        0x2e8a => format!("Raspberry Pi Board (VID: {:04X}, PID: {:04X})", vid, pid),
        0x239a => format!("Adafruit Board (VID: {:04X}, PID: {:04X})", vid, pid),
        0x1b4f => format!("SparkFun Board (VID: {:04X}, PID: {:04X})", vid, pid),
        0x0403 => "FTDI USB-Serial Converter".to_string(),
        0x10c4 => "Silicon Labs USB-Serial".to_string(),
        0x1a86 => "CH340/CH341 USB-Serial".to_string(),
        0x067b => "Prolific USB-Serial".to_string(),
        _ => format!("USB Serial Device (VID: {:04X}, PID: {:04X})", vid, pid)
    }
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
        }
    }
}

#[godot_api]
impl GdSerial {
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
    pub fn get_port_device_name(&self, port_name: GString) -> GString {
        let port_name_str = port_name.to_string();
        
        match serialport::available_ports() {
            Ok(ports) => {
                for port in ports {
                    if port.port_name == port_name_str {
                        let device_name = match &port.port_type {
                            SerialPortType::UsbPort(usb_info) => {
                                get_usb_device_name(
                                    usb_info.vid, 
                                    usb_info.pid, 
                                    &usb_info.manufacturer, 
                                    &usb_info.product
                                )
                            }
                            SerialPortType::PciPort => "PCI Serial Port".to_string(),
                            SerialPortType::BluetoothPort => "Bluetooth Serial Port".to_string(),
                            SerialPortType::Unknown => "Unknown Serial Device".to_string(),
                        };
                        return GString::from(device_name);
                    }
                }
                // Port not found, return the port name itself
                port_name
            }
            Err(_) => {
                // Error listing ports, return the port name itself
                port_name
            }
        }
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
            .open()
        {
            Ok(port) => {
                self.port = Some(port);
                // Port opened successfully - removed print output per issue #1
                true
            }
            Err(e) => {
                godot_error!("Failed to open port {}: {}", self.port_name, e);
                false
            }
        }
    }
    
    #[func]
    pub fn close(&mut self) {
        if self.port.is_some() {
            self.port = None;
            // Port closed - removed print output per issue #1
        }
    }
    
    #[func]
    pub fn is_open(&self) -> bool {
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
                                godot_error!("Failed to flush port: {}", e);
                                false
                            }
                        }
                    }
                    Err(e) => {
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
        match &mut self.port {
            Some(port) => {
                let mut buffer = vec![0; size as usize];
                match port.read(&mut buffer) {
                    Ok(bytes_read) => {
                        buffer.truncate(bytes_read);
                        PackedByteArray::from(&buffer[..])
                    }
                    Err(e) => {
                        godot_error!("Failed to read from port: {}", e);
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
        match &mut self.port {
            Some(port) => {
                let mut line = String::new();
                let mut byte = [0u8; 1];
                
                loop {
                    match port.read_exact(&mut byte) {
                        Ok(_) => {
                            let ch = byte[0] as char;
                            if ch == '\n' {
                                break;
                            } else if ch != '\r' {
                                line.push(ch);
                            }
                        }
                        Err(e) => {
                            if line.is_empty() {
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
        match &mut self.port {
            Some(port) => {
                match port.bytes_to_read() {
                    Ok(bytes) => bytes as u32,
                    Err(e) => {
                        godot_error!("Failed to get available bytes: {}", e);
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