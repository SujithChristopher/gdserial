use godot::prelude::*;
use parking_lot::Mutex;
use serialport::{DataBits, ErrorKind, FlowControl, Parity, SerialPort, SerialPortType, StopBits};
use std::io::{self, Read};
use std::sync::Arc;
use std::time::Duration;

fn get_usb_device_name(
    vid: u16,
    pid: u16,
    manufacturer: &Option<String>,
    product: &Option<String>,
) -> String {
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

/// Internal state for GdSerial - all mutable fields protected by Mutex
struct GdSerialState {
    port: Option<Box<dyn SerialPort>>,
    port_name: String,
    baud_rate: u32,
    data_bits: DataBits,
    stop_bits: StopBits,
    parity: Parity,
    flow_control: FlowControl,
    timeout: Duration,
    is_connected: bool,
}

/// Cross-platform serial communication for Godot 4
///
/// # Thread Safety
///
/// As of version 0.3.0, GdSerial is thread-safe and can be safely accessed
/// from multiple threads simultaneously. All operations are automatically
/// synchronized using interior mutability with Arc<Mutex<>>.
///
/// ## Usage from Background Threads
///
/// ```gdscript
/// # Create serial port on main thread
/// var serial = GdSerial.new()
/// serial.set_port("COM3")
/// serial.open()
///
/// # Access from background thread (Godot Thread)
/// var thread = Thread.new()
/// thread.start(func():
///     while serial.is_open():
///         var data = serial.readline()
///         # Process data...
/// )
/// ```
///
/// ## Godot Threading Best Practices
///
/// - Use Godot's Thread class for background tasks
/// - Be careful with Godot API calls from background threads
/// - Use call_deferred to communicate back to main thread
/// - Always join threads before freeing the serial port
#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct GdSerial {
    base: Base<RefCounted>,
    // Thread-safe state using Arc<Mutex<>>
    // Arc allows shared ownership across clones
    // Mutex ensures exclusive access to mutable state
    state: Arc<Mutex<GdSerialState>>,
}

#[godot_api]
impl IRefCounted for GdSerial {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            state: Arc::new(Mutex::new(GdSerialState {
                port: None,
                port_name: String::new(),
                baud_rate: 9600,
                data_bits: DataBits::Eight,
                stop_bits: StopBits::One,
                flow_control: FlowControl::None,
                parity: Parity::None,
                timeout: Duration::from_millis(1000),
                is_connected: false,
            })),
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
                matches!(
                    io_error,
                    io::ErrorKind::BrokenPipe
                        | io::ErrorKind::ConnectionAborted
                        | io::ErrorKind::NotConnected
                        | io::ErrorKind::UnexpectedEof
                        | io::ErrorKind::PermissionDenied // Can occur on disconnect
                )
            }
            _ => false,
        }
    }

    /// Check if IO error indicates disconnection
    fn is_io_disconnection_error(error: &io::Error) -> bool {
        matches!(
            error.kind(),
            io::ErrorKind::BrokenPipe
                | io::ErrorKind::ConnectionAborted
                | io::ErrorKind::NotConnected
                | io::ErrorKind::UnexpectedEof
                | io::ErrorKind::PermissionDenied
        )
    }

    /// Internal helper: Handle potential disconnection by closing the port if device is no longer available
    /// Operates on already-locked state
    fn handle_potential_disconnection_internal(
        state: &mut GdSerialState,
        error: &serialport::Error,
    ) {
        if Self::is_disconnection_error(error) {
            godot_print!("Device disconnected, closing port");
            state.port = None;
            state.is_connected = false;
        }
    }

    /// Internal helper: Handle potential disconnection for IO errors
    /// Operates on already-locked state
    fn handle_potential_io_disconnection_internal(state: &mut GdSerialState, error: &io::Error) {
        if Self::is_io_disconnection_error(error) {
            godot_print!("Device disconnected (IO error), closing port");
            state.port = None;
            state.is_connected = false;
        }
    }

    /// Internal helper: Actively test if the port is still connected by attempting a non-destructive operation
    /// Operates on already-locked state
    fn test_connection_internal(state: &mut GdSerialState) -> bool {
        if let Some(ref mut port) = state.port {
            // Try bytes_to_read as the primary test - it's the most reliable
            match port.bytes_to_read() {
                Ok(_) => true,
                Err(e) => {
                    // Any error here likely means disconnection
                    godot_print!("Connection test failed: {} - marking as disconnected", e);
                    state.port = None;
                    state.is_connected = false;
                    false
                }
            }
        } else {
            false
        }
    }

    #[func]
    pub fn list_ports(&self) -> VarDictionary {
        let mut ports_dict = VarDictionary::new();

        match serialport::available_ports() {
            Ok(ports) => {
                for (i, port) in ports.iter().enumerate() {
                    let mut port_info = VarDictionary::new();
                    port_info.set(GString::from("port_name"), GString::from(&port.port_name));

                    let (port_type, device_name) = match &port.port_type {
                        SerialPortType::UsbPort(usb_info) => {
                            let port_type = format!(
                                "USB - VID: {:04X}, PID: {:04X}",
                                usb_info.vid, usb_info.pid
                            );
                            let device_name = get_usb_device_name(
                                usb_info.vid,
                                usb_info.pid,
                                &usb_info.manufacturer,
                                &usb_info.product,
                            );
                            (port_type, device_name)
                        }
                        SerialPortType::PciPort => {
                            ("PCI".to_string(), "PCI Serial Port".to_string())
                        }
                        SerialPortType::BluetoothPort => {
                            ("Bluetooth".to_string(), "Bluetooth Serial Port".to_string())
                        }
                        SerialPortType::Unknown => {
                            ("Unknown".to_string(), "Unknown Serial Device".to_string())
                        }
                    };

                    port_info.set(GString::from("port_type"), GString::from(&port_type));
                    port_info.set(GString::from("device_name"), GString::from(&device_name));
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
    pub fn set_port(&self, port_name: GString) {
        let mut state = self.state.lock();
        state.port_name = port_name.to_string();
    }

    #[func]
    pub fn set_baud_rate(&self, baud_rate: u32) {
        let mut state = self.state.lock();
        state.baud_rate = baud_rate;
    }

    #[func]
    pub fn set_data_bits(&self, data_bits: u8) {
        let mut state = self.state.lock();
        match data_bits {
            6 => {
                state.data_bits = DataBits::Six;
            }
            7 => {
                state.data_bits = DataBits::Seven;
            }
            8 => {
                state.data_bits = DataBits::Eight;
            }
            _ => {
                godot_error!("Data bits must be between 6 and 8")
            }
        }
    }

    #[func]
    pub fn set_parity(&self, parity: bool) {
        let mut state = self.state.lock();
        match parity {
            false => {
                state.parity = Parity::None;
            }
            true => {
                state.parity = Parity::Odd;
            }
        }
    }

    #[func]
    pub fn set_stop_bits(&self, stop_bits: u8) {
        let mut state = self.state.lock();
        match stop_bits {
            1 => {
                state.stop_bits = StopBits::One;
            }
            2 => {
                state.stop_bits = StopBits::Two;
            }
            _ => {
                godot_error!("Stop bits must be between 1 and 2")
            }
        }
    }

    #[func]
    pub fn set_flow_control(&self, flow_control: u8) {
        let mut state = self.state.lock();
        match flow_control {
            0 => {
                state.flow_control = FlowControl::None;
            }
            1 => {
                state.flow_control = FlowControl::Software;
            }
            2 => {
                state.flow_control = FlowControl::Hardware;
            }
            _ => {
                godot_error!("Data bits must be between 0 and 2")
            }
        }
    }

    #[func]
    pub fn set_timeout(&self, timeout_ms: u32) {
        let mut state = self.state.lock();
        state.timeout = Duration::from_millis(timeout_ms as u64);
    }

    #[func]
    pub fn open(&self) -> bool {
        let mut state = self.state.lock();

        if state.port_name.is_empty() {
            godot_error!("Port name not set");
            return false;
        }

        match serialport::new(&state.port_name, state.baud_rate)
            .timeout(state.timeout)
            .data_bits(state.data_bits)
            .parity(state.parity)
            .stop_bits(state.stop_bits)
            .flow_control(state.flow_control)
            .open()
        {
            Ok(port) => {
                state.port = Some(port);
                state.is_connected = true;
                true
            }
            Err(e) => {
                godot_error!("Failed to open port {}: {}", state.port_name, e);
                state.is_connected = false;
                false
            }
        }
    }

    #[func]
    pub fn close(&self) {
        let mut state = self.state.lock();
        if state.port.is_some() {
            state.port = None;
            state.is_connected = false;
        }
    }

    #[func]
    pub fn is_open(&self) -> bool {
        // Always test the actual connection state
        let mut state = self.state.lock();
        if state.port.is_some() {
            Self::test_connection_internal(&mut state)
        } else {
            false
        }
    }

    #[func]
    pub fn write(&self, data: PackedByteArray) -> bool {
        let mut state = self.state.lock();

        // First check if connected
        if !Self::test_connection_internal(&mut state) {
            godot_error!("Port not connected");
            return false;
        }

        match &mut state.port {
            Some(port) => {
                let bytes = data.to_vec();
                match port.write_all(&bytes) {
                    Ok(_) => match port.flush() {
                        Ok(_) => true,
                        Err(e) => {
                            Self::handle_potential_io_disconnection_internal(&mut state, &e);
                            godot_error!("Failed to flush port: {}", e);
                            false
                        }
                    },
                    Err(e) => {
                        Self::handle_potential_io_disconnection_internal(&mut state, &e);
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
    pub fn write_string(&self, data: GString) -> bool {
        let bytes = data.to_string().into_bytes();
        let packed_bytes = PackedByteArray::from(&bytes[..]);
        self.write(packed_bytes)
    }

    #[func]
    pub fn writeline(&self, data: GString) -> bool {
        let data_with_newline = format!("{}\n", data.to_string());
        let bytes = data_with_newline.into_bytes();
        let packed_bytes = PackedByteArray::from(&bytes[..]);
        self.write(packed_bytes)
    }

    #[func]
    pub fn read(&self, size: u32) -> PackedByteArray {
        let mut state = self.state.lock();

        // First check if connected
        if !Self::test_connection_internal(&mut state) {
            return PackedByteArray::new();
        }

        match &mut state.port {
            Some(port) => {
                let mut buffer = vec![0; size as usize];
                match port.read(&mut buffer) {
                    Ok(bytes_read) => {
                        buffer.truncate(bytes_read);
                        PackedByteArray::from(&buffer[..])
                    }
                    Err(e) => {
                        // Don't treat timeout as disconnection
                        if e.kind() != io::ErrorKind::TimedOut
                            && e.kind() != io::ErrorKind::WouldBlock
                        {
                            Self::handle_potential_io_disconnection_internal(&mut state, &e);
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
    pub fn read_string(&self, size: u32) -> GString {
        let bytes = self.read(size);
        match String::from_utf8(bytes.to_vec()) {
            Ok(string) => GString::from(&string),
            Err(e) => {
                godot_error!("Failed to convert bytes to string: {}", e);
                GString::new()
            }
        }
    }

    #[func]
    pub fn readline(&self) -> GString {
        let mut state = self.state.lock();

        // First check if connected
        if !Self::test_connection_internal(&mut state) {
            return GString::new();
        }

        match &mut state.port {
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
                                Self::handle_potential_io_disconnection_internal(&mut state, &e);
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

                GString::from(&line)
            }
            None => {
                godot_error!("Port not open");
                GString::new()
            }
        }
    }

    #[func]
    pub fn bytes_available(&self) -> u32 {
        let mut state = self.state.lock();

        // First check if connected
        if !Self::test_connection_internal(&mut state) {
            return 0;
        }

        match &mut state.port {
            Some(port) => {
                match port.bytes_to_read() {
                    Ok(bytes) => bytes as u32,
                    Err(e) => {
                        // Any error in bytes_to_read likely means the port is in a bad state
                        // Mark as disconnected regardless of error type
                        godot_error!(
                            "Failed to get available bytes: {} - marking port as disconnected",
                            e
                        );
                        state.port = None;
                        state.is_connected = false;
                        0
                    }
                }
            }
            None => 0,
        }
    }

    #[func]
    pub fn clear_buffer(&self) -> bool {
        let mut state = self.state.lock();

        // First check if connected
        if !Self::test_connection_internal(&mut state) {
            return false;
        }

        match &mut state.port {
            Some(port) => match port.clear(serialport::ClearBuffer::All) {
                Ok(_) => true,
                Err(e) => {
                    Self::handle_potential_disconnection_internal(&mut state, &e);
                    godot_error!("Failed to clear buffer: {}", e);
                    false
                }
            },
            None => {
                godot_error!("Port not open");
                false
            }
        }
    }
}
