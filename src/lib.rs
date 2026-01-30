use godot::prelude::*;
use serialport::{DataBits, ErrorKind, FlowControl, Parity, SerialPort, SerialPortType, StopBits};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
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

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct GdSerial {
    base: Base<RefCounted>,
    // Wrapped in Arc<Mutex<...>> so the handle can be shared safely across threads
    port: Option<Arc<Mutex<Box<dyn SerialPort>>>>,
    port_name: String,
    baud_rate: u32,
    data_bits: DataBits,
    stop_bits: StopBits,
    parity: Parity,
    flow_control: FlowControl,
    timeout: Duration,
    is_connected: bool, // Track connection state
}

// Message coming from reader thread to Godot thread
enum ReaderEvent {
    Data(String, Vec<u8>), // (port_name, data)
    Disconnected(String),   // port_name
}

// Buffering modes for the reader thread
#[derive(Clone, Copy)]
enum BufferingMode {
    Raw,               // 0: Emit all chunks immediately
    LineBuffered,      // 1: Wait for \n
    CustomDelimiter(u8), // 2: Wait for custom delimiter
}

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct GdSerialManager {
    base: Base<RefCounted>,
    ports: Mutex<HashMap<String, Arc<Mutex<Box<dyn SerialPort>>>>>,
    reader_handles: Mutex<HashMap<String, thread::JoinHandle<()>>>,
    stop_flags: Mutex<HashMap<String, Arc<AtomicBool>>>,
    port_modes: Mutex<HashMap<String, BufferingMode>>,
    tx: Mutex<Option<mpsc::Sender<ReaderEvent>>>,
    rx: Mutex<Option<mpsc::Receiver<ReaderEvent>>>,
}

#[godot_api]
impl IRefCounted for GdSerialManager {
    fn init(base: Base<RefCounted>) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            base,
            ports: Mutex::new(HashMap::new()),
            reader_handles: Mutex::new(HashMap::new()),
            stop_flags: Mutex::new(HashMap::new()),
            port_modes: Mutex::new(HashMap::new()),
            tx: Mutex::new(Some(tx)),
            rx: Mutex::new(Some(rx)),
        }
    }
}

#[godot_api]
impl GdSerialManager {
    #[signal]
    fn data_received(port: GString, data: PackedByteArray);
    #[signal]
    fn port_disconnected(port: GString);

    /// List available ports (same structure as GdSerial::list_ports)
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

    fn spawn_reader_thread(
        &self,
        port_name: String,
        port_arc: Arc<Mutex<Box<dyn SerialPort>>>,
        stop_flag: Arc<AtomicBool>,
        mode: BufferingMode,
    ) -> Option<thread::JoinHandle<()>> {
        let tx_opt = self.tx.lock().ok()?.clone();
        let tx = tx_opt?;

        Some(thread::spawn(move || {
            let mut read_buffer = [0u8; 1024];
            let mut line_buffer = Vec::new();

            loop {
                if stop_flag.load(Ordering::Relaxed) {
                    break;
                }

                let read_result = {
                    match port_arc.lock() {
                        Ok(mut port) => port.read(&mut read_buffer),
                        Err(_) => return,
                    }
                };

                match read_result {
                    Ok(0) => {
                        // No data this tick, continue
                    }
                    Ok(n) => {
                        match mode {
                            BufferingMode::Raw => {
                                // Mode 0: Emit immediately without buffering
                                let _ = tx.send(ReaderEvent::Data(
                                    port_name.clone(),
                                    read_buffer[..n].to_vec(),
                                ));
                            }
                            BufferingMode::LineBuffered => {
                                // Mode 1: Buffer until newline is found
                                for &byte in &read_buffer[..n] {
                                    line_buffer.push(byte);
                                    if byte == b'\n' {
                                        // Found complete line, emit and clear
                                        let _ = tx.send(ReaderEvent::Data(
                                            port_name.clone(),
                                            line_buffer.clone(),
                                        ));
                                        line_buffer.clear();
                                    }
                                }
                            }
                            BufferingMode::CustomDelimiter(delim) => {
                                // Mode 2: Buffer until custom delimiter is found
                                for &byte in &read_buffer[..n] {
                                    line_buffer.push(byte);
                                    if byte == delim {
                                        // Found delimiter, emit and clear
                                        let _ = tx.send(ReaderEvent::Data(
                                            port_name.clone(),
                                            line_buffer.clone(),
                                        ));
                                        line_buffer.clear();
                                    }
                                }
                            }
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                        // Timeout: Emit any buffered data for non-raw modes
                        if !line_buffer.is_empty() && !matches!(mode, BufferingMode::Raw) {
                            let _ = tx.send(ReaderEvent::Data(
                                port_name.clone(),
                                line_buffer.clone(),
                            ));
                            line_buffer.clear();
                        }
                    }
                    Err(_) => {
                        let _ = tx.send(ReaderEvent::Disconnected(port_name.clone()));
                        break;
                    }
                }

                // Small sleep to avoid busy loop on WouldBlock if timeout == 0
                std::thread::sleep(Duration::from_millis(5));
            }
        }))
    }

    #[func]
    pub fn open_port(&mut self, port_name: GString, baud_rate: i32, timeout_ms: i32, mode: i32) -> bool {
        let port_name_str = port_name.to_string();
        // Close existing instance if any
        self.close_port(port_name.clone());

        // Convert mode int to BufferingMode (default to LineBuffered if invalid)
        let buffering_mode = match mode {
            0 => BufferingMode::Raw,
            2 => BufferingMode::CustomDelimiter(b'\n'), // Default delimiter for mode 2
            _ => BufferingMode::LineBuffered, // mode 1 and invalid values
        };

        let builder = serialport::new(&port_name_str, baud_rate as u32)
            .timeout(Duration::from_millis(timeout_ms as u64));

        match builder.open() {
            Ok(port) => {
                let port_arc = Arc::new(Mutex::new(port));
                let stop_flag = Arc::new(AtomicBool::new(false));

                if let Ok(mut map) = self.ports.lock() {
                    map.insert(port_name_str.clone(), port_arc.clone());
                }
                if let Ok(mut map) = self.stop_flags.lock() {
                    map.insert(port_name_str.clone(), stop_flag.clone());
                }
                if let Ok(mut map) = self.port_modes.lock() {
                    map.insert(port_name_str.clone(), buffering_mode);
                }

                if let Some(handle) =
                    self.spawn_reader_thread(port_name_str.clone(), port_arc, stop_flag, buffering_mode)
                {
                    if let Ok(mut hmap) = self.reader_handles.lock() {
                        hmap.insert(port_name_str, handle);
                    }
                }
                true
            }
            Err(e) => {
                godot_error!("Failed to open port {}: {}", port_name, e);
                false
            }
        }
    }

    #[func]
    pub fn close_port(&mut self, port_name: GString) {
        let name = port_name.to_string();
        if let Ok(mut flags) = self.stop_flags.lock() {
            if let Some(flag) = flags.remove(&name) {
                flag.store(true, Ordering::Relaxed);
            }
        }

        if let Ok(mut handles) = self.reader_handles.lock() {
            if let Some(handle) = handles.remove(&name) {
                let _ = handle.join();
            }
        }

        if let Ok(mut ports) = self.ports.lock() {
            ports.remove(&name);
        }

        if let Ok(mut modes) = self.port_modes.lock() {
            modes.remove(&name);
        }
    }

    #[func]
    pub fn is_open(&self, port_name: GString) -> bool {
        if let Ok(ports) = self.ports.lock() {
            ports.contains_key(&port_name.to_string())
        } else {
            false
        }
    }

    /// Set custom delimiter for a port using mode 2 (CustomDelimiter)
    #[func]
    pub fn set_delimiter(&mut self, port_name: GString, delimiter: i32) -> bool {
        let name = port_name.to_string();
        if let Ok(mut modes) = self.port_modes.lock() {
            if let Some(mode) = modes.get_mut(&name) {
                *mode = BufferingMode::CustomDelimiter(delimiter as u8);
                return true;
            }
        }
        godot_error!("Port not found: {}", port_name);
        false
    }

    #[func]
    pub fn write_port(&self, port_name: GString, data: PackedByteArray) -> bool {
        let Some(port_arc) = self
            .ports
            .lock()
            .ok()
            .and_then(|m| m.get(&port_name.to_string()).cloned())
        else {
            godot_error!("Port not open");
            return false;
        };

        let write_result = {
            match port_arc.lock() {
                Ok(mut port) => {
                    let bytes = data.to_vec();
                    match port.write_all(&bytes) {
                        Ok(_) => match port.flush() {
                            Ok(_) => true,
                            Err(e) => {
                                godot_error!("Failed to flush port {}: {}", port_name, e);
                                false
                            }
                        },
                        Err(e) => {
                            godot_error!("Failed to write to port {}: {}", port_name, e);
                            false
                        }
                    }
                }
                Err(e) => {
                    godot_error!("Port mutex poisoned: {}", e);
                    false
                }
            }
        };

        write_result
    }

    #[func]
    pub fn reconfigure_port(
        &self,
        port_name: GString,
        baud_rate: i32,
        data_bits: u8,
        parity: i32,
        stop_bits: u8,
        flow_control: u8,
        timeout_ms: i32,
    ) -> bool {
        let Some(port_arc) = self
            .ports
            .lock()
            .ok()
            .and_then(|m| m.get(&port_name.to_string()).cloned())
        else {
            godot_error!("Port not open");
            return false;
        };

        let mut port = match port_arc.lock() {
            Ok(p) => p,
            Err(e) => {
                godot_error!("Port mutex poisoned: {}", e);
                return false;
            }
        };

        // Apply settings; log errors but try to continue
        let mut ok = true;
        if let Err(e) = port.set_baud_rate(baud_rate as u32) {
            ok = false;
            godot_error!("Failed to set baud_rate: {}", e);
        }
        if let Err(e) = port.set_timeout(Duration::from_millis(timeout_ms as u64)) {
            ok = false;
            godot_error!("Failed to set timeout: {}", e);
        }
        let db = match data_bits {
            6 => DataBits::Six,
            7 => DataBits::Seven,
            _ => DataBits::Eight,
        };
        if let Err(e) = port.set_data_bits(db) {
            ok = false;
            godot_error!("Failed to set data bits: {}", e);
        }
        let pb = match parity {
            1 => Parity::Odd,
            2 => Parity::Even,
            _ => Parity::None,
        };
        if let Err(e) = port.set_parity(pb) {
            ok = false;
            godot_error!("Failed to set parity: {}", e);
        }
        let sb = match stop_bits {
            2 => StopBits::Two,
            _ => StopBits::One,
        };
        if let Err(e) = port.set_stop_bits(sb) {
            ok = false;
            godot_error!("Failed to set stop bits: {}", e);
        }
        let fc = match flow_control {
            1 => FlowControl::Software,
            2 => FlowControl::Hardware,
            _ => FlowControl::None,
        };
        if let Err(e) = port.set_flow_control(fc) {
            ok = false;
            godot_error!("Failed to set flow control: {}", e);
        }
        ok
    }

    /// Drain async reader queue, emit signals, and also return events as array of dictionaries
    #[func]
    pub fn poll_events(&mut self) -> Array<VarDictionary> {
        let mut out: Array<VarDictionary> = Array::new();
        let mut events = Vec::new();

        if let Ok(rx_opt) = self.rx.lock() {
            if let Some(rx) = rx_opt.as_ref() {
                loop {
                    match rx.try_recv() {
                        Ok(ev) => events.push(ev),
                        Err(mpsc::TryRecvError::Empty) => break,
                        Err(mpsc::TryRecvError::Disconnected) => break,
                    }
                }
            }
        }

        for ev in events {
            match ev {
                ReaderEvent::Data(port, data) => {
                    let gport = GString::from(&port);
                    let pba = PackedByteArray::from(&data[..]);

                    // Emit the data_received signal
                    self.base_mut().emit_signal(
                        &StringName::from("data_received"),
                        &[gport.to_variant(), pba.to_variant()],
                    );

                    let mut dict = VarDictionary::new();
                    dict.set(GString::from("port"), gport);
                    dict.set(GString::from("data"), pba);
                    out.push(&dict);
                }
                ReaderEvent::Disconnected(port) => {
                    let gport = GString::from(&port);

                    // Emit the port_disconnected signal
                    self.base_mut().emit_signal(
                        &StringName::from("port_disconnected"),
                        &[gport.to_variant()],
                    );

                    let mut dict = VarDictionary::new();
                    dict.set(GString::from("port"), gport.clone());
                    dict.set(GString::from("disconnected"), true);
                    out.push(&dict);
                    // Clean internal state
                    self.close_port(GString::from(port.as_str()));
                }
            }
        }

        out
    }
}

#[godot_api]
impl IRefCounted for GdSerial {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            port: None,
            port_name: String::new(),
            baud_rate: 9600,
            data_bits: DataBits::Eight,
            stop_bits: StopBits::One,
            flow_control: FlowControl::None,
            parity: Parity::None,
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
        let Some(port_arc) = self.port.as_ref().map(Arc::clone) else {
            return false;
        };

        let connected = match port_arc.lock() {
            Ok(port) => match port.bytes_to_read() {
                Ok(_) => true,
                Err(e) => {
                    // Any error here likely means disconnection
                    godot_print!("Connection test failed: {} - marking as disconnected", e);
                    false
                }
            },
            Err(e) => {
                godot_error!("Port mutex poisoned: {}", e);
                false
            }
        };

        if !connected {
            self.port = None;
            self.is_connected = false;
        }

        connected
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
    pub fn set_port(&mut self, port_name: GString) {
        self.port_name = port_name.to_string();
    }

    #[func]
    pub fn set_baud_rate(&mut self, baud_rate: u32) {
        self.baud_rate = baud_rate;
    }

    #[func]
    pub fn set_data_bits(&mut self, data_bits: u8) {
        match data_bits {
            6 => {
                self.data_bits = DataBits::Six;
            }
            7 => {
                self.data_bits = DataBits::Seven;
            }
            8 => {
                self.data_bits = DataBits::Eight;
            }
            _ => {
                godot_error!("Data bits must be between 6 and 8")
            }
        }
    }

    #[func]
    pub fn set_parity(&mut self, parity: i32) {
        match parity {
            1 => {
                self.parity = Parity::Odd;
            }
            2 => {
                self.parity = Parity::Even;
            }
            _ => {
                self.parity = Parity::None;
            }
        }
    }

    #[func]
    pub fn set_stop_bits(&mut self, stop_bits: u8) {
        match stop_bits {
            1 => {
                self.stop_bits = StopBits::One;
            }
            2 => {
                self.stop_bits = StopBits::Two;
            }
            _ => {
                godot_error!("Stop bits must be between 1 and 2")
            }
        }
    }

    #[func]
    pub fn set_flow_control(&mut self, flow_control: u8) {
        match flow_control {
            0 => {
                self.flow_control = FlowControl::None;
            }
            1 => {
                self.flow_control = FlowControl::Software;
            }
            2 => {
                self.flow_control = FlowControl::Hardware;
            }
            _ => {
                godot_error!("Data bits must be between 0 and 2")
            }
        }
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
            .data_bits(self.data_bits)
            .parity(self.parity)
            .stop_bits(self.stop_bits)
            .flow_control(self.flow_control)
            .open()
        {
            Ok(port) => {
                self.port = Some(Arc::new(Mutex::new(port)));
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

        let Some(port_arc) = self.port.as_ref().map(Arc::clone) else {
            godot_error!("Port not open");
            return false;
        };

        let write_result = {
            match port_arc.lock() {
                Ok(mut port) => {
                    let bytes = data.to_vec();
                    match port.write_all(&bytes) {
                        Ok(_) => match port.flush() {
                            Ok(_) => true,
                            Err(e) => {
                                self.handle_potential_io_disconnection(&e);
                                godot_error!("Failed to flush port: {}", e);
                                false
                            }
                        },
                        Err(e) => {
                            self.handle_potential_io_disconnection(&e);
                            godot_error!("Failed to write to port: {}", e);
                            false
                        }
                    }
                }
                Err(e) => {
                    godot_error!("Port mutex poisoned: {}", e);
                    self.port = None;
                    self.is_connected = false;
                    false
                }
            }
        };

        write_result
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

        let Some(port_arc) = self.port.as_ref().map(Arc::clone) else {
            godot_error!("Port not open");
            return PackedByteArray::new();
        };

        let read_result = {
            match port_arc.lock() {
                Ok(mut port) => {
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
                                self.handle_potential_io_disconnection(&e);
                                godot_error!("Failed to read from port: {}", e);
                            }
                            PackedByteArray::new()
                        }
                    }
                }
                Err(e) => {
                    godot_error!("Port mutex poisoned: {}", e);
                    self.port = None;
                    self.is_connected = false;
                    PackedByteArray::new()
                }
            }
        };

        read_result
    }

    #[func]
    pub fn read_string(&mut self, size: u32) -> GString {
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
    pub fn readline(&mut self) -> GString {
        // First check if connected
        if !self.test_connection() {
            return GString::new();
        }

        let Some(port_arc) = self.port.as_ref().map(Arc::clone) else {
            godot_error!("Port not open");
            return GString::new();
        };

        let line_result = {
            match port_arc.lock() {
                Ok(mut port) => {
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

                    GString::from(&line)
                }
                Err(e) => {
                    godot_error!("Port mutex poisoned: {}", e);
                    self.port = None;
                    self.is_connected = false;
                    GString::new()
                }
            }
        };

        line_result
    }

    #[func]
    pub fn bytes_available(&mut self) -> u32 {
        // First check if connected
        if !self.test_connection() {
            return 0;
        }

        let Some(port_arc) = self.port.as_ref().map(Arc::clone) else {
            return 0;
        };

        let bytes_result = {
            match port_arc.lock() {
                Ok(port) => match port.bytes_to_read() {
                    Ok(bytes) => bytes as u32,
                    Err(e) => {
                        // Any error in bytes_to_read likely means the port is in a bad state
                        // Mark as disconnected regardless of error type
                        godot_error!(
                            "Failed to get available bytes: {} - marking port as disconnected",
                            e
                        );
                        self.port = None;
                        self.is_connected = false;
                        0
                    }
                },
                Err(e) => {
                    godot_error!("Port mutex poisoned: {}", e);
                    self.port = None;
                    self.is_connected = false;
                    0
                }
            }
        };

        bytes_result
    }

    #[func]
    pub fn clear_buffer(&mut self) -> bool {
        // First check if connected
        if !self.test_connection() {
            return false;
        }

        let Some(port_arc) = self.port.as_ref().map(Arc::clone) else {
            godot_error!("Port not open");
            return false;
        };

        let clear_result = {
            match port_arc.lock() {
                Ok(port) => match port.clear(serialport::ClearBuffer::All) {
                    Ok(_) => true,
                    Err(e) => {
                        self.handle_potential_disconnection(&e);
                        godot_error!("Failed to clear buffer: {}", e);
                        false
                    }
                },
                Err(e) => {
                    godot_error!("Port mutex poisoned: {}", e);
                    self.port = None;
                    self.is_connected = false;
                    false
                }
            }
        };

        clear_result
    }
}
