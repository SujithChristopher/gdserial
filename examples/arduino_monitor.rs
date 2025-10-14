// Arduino continuous monitor - reads data continuously from serial port
// Usage: cargo run --example arduino_monitor COM10

use serialport::available_ports;
use std::io::{self, Read};
use std::time::Duration;
use std::env;

fn main() {
    println!("=== Arduino Serial Monitor ===\n");

    // Get port from command line
    let port_name = if let Some(port_arg) = env::args().nth(1) {
        port_arg
    } else {
        // List available ports
        println!("Available ports:");
        match available_ports() {
            Ok(ports) => {
                for port in ports.iter() {
                    println!("  - {}", port.port_name);
                }
            }
            Err(e) => {
                eprintln!("Error listing ports: {}", e);
            }
        }
        eprintln!("\nUsage: cargo run --example arduino_monitor <PORT>");
        eprintln!("Example: cargo run --example arduino_monitor COM10");
        return;
    };

    println!("Opening port: {}", port_name);

    // Open port with 9600 baud (common Arduino default)
    let mut port = match serialport::new(&port_name, 9600)
        .timeout(Duration::from_millis(100))  // Short timeout for responsive reading
        .open()
    {
        Ok(p) => {
            println!("✓ Port opened successfully at 9600 baud\n");
            p
        }
        Err(e) => {
            eprintln!("✗ Failed to open port: {}", e);
            return;
        }
    };

    // Wait a moment for Arduino to reset (some Arduinos reset on serial connection)
    println!("Waiting 2 seconds for Arduino to initialize...\n");
    std::thread::sleep(Duration::from_secs(2));

    // Clear any initialization garbage
    let _ = port.clear(serialport::ClearBuffer::All);

    println!("--- Monitoring serial output (Press Ctrl+C to exit) ---\n");

    let mut buffer = [0u8; 256];
    let mut line_buffer = Vec::new();
    let mut byte_count = 0;

    loop {
        // Check how many bytes are available first
        match port.bytes_to_read() {
            Ok(bytes_available) => {
                if bytes_available > 0 {
                    print!("[{} bytes available] ", bytes_available);
                    io::Write::flush(&mut io::stdout()).unwrap();
                }
            }
            Err(e) => {
                eprintln!("\n✗ Error checking bytes_to_read: {}", e);
                eprintln!("This might indicate the device was disconnected");
                break;
            }
        }

        // Try to read data
        match port.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    byte_count += bytes_read;

                    // Process each byte
                    for i in 0..bytes_read {
                        let byte = buffer[i];

                        if byte == b'\n' {
                            // End of line - print the accumulated line
                            let line = String::from_utf8_lossy(&line_buffer);
                            println!("📥 {}", line.trim_end_matches('\r'));
                            line_buffer.clear();
                        } else {
                            line_buffer.push(byte);
                        }
                    }

                    // Also show raw bytes if not printable ASCII
                    let raw_data = &buffer[..bytes_read];
                    if raw_data.iter().any(|&b| b < 32 && b != b'\r' && b != b'\n' && b != b'\t') {
                        print!("   [Raw bytes: ");
                        for &b in raw_data {
                            print!("{:02X} ", b);
                        }
                        println!("]");
                    }
                }
            }
            Err(e) if e.kind() == io::ErrorKind::TimedOut => {
                // Timeout is normal when no data is available
                // Just continue to next iteration
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(e) => {
                eprintln!("\n✗ Read error: {}", e);
                eprintln!("Error kind: {:?}", e.kind());
                break;
            }
        }
    }

    println!("\n--- Monitoring stopped ---");
    println!("Total bytes received: {}", byte_count);
}
