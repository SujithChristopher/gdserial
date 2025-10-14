// Standalone test program for gdserial functionality
// Compile with: rustc test_serial.rs --edition 2021 -L target/release/deps
// Or run with: cargo run --example test_serial

use serialport::{SerialPort, available_ports};
use std::io::{self, Read, Write};
use std::time::Duration;

fn main() {
    println!("=== Serial Port Test Program ===\n");

    // Test 1: List available ports
    println!("1. Listing available ports...");
    match available_ports() {
        Ok(ports) => {
            if ports.is_empty() {
                println!("   No serial ports found!");
                return;
            }
            for (i, port) in ports.iter().enumerate() {
                println!("   [{}] {}", i, port.port_name);
            }
        }
        Err(e) => {
            eprintln!("   Error listing ports: {}", e);
            return;
        }
    }

    // Get user to select a port
    println!("\nEnter port number to test (or port name like COM3): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let port_name = input.trim();

    // Parse port selection
    let selected_port = if let Ok(idx) = port_name.parse::<usize>() {
        match available_ports() {
            Ok(ports) => {
                if idx < ports.len() {
                    ports[idx].port_name.clone()
                } else {
                    eprintln!("Invalid port number!");
                    return;
                }
            }
            Err(_) => return,
        }
    } else {
        port_name.to_string()
    };

    println!("\n2. Opening port: {}", selected_port);

    // Test 2: Open port
    let mut port = match serialport::new(&selected_port, 9600)
        .timeout(Duration::from_millis(1000))
        .open()
    {
        Ok(p) => {
            println!("   ✓ Port opened successfully");
            p
        }
        Err(e) => {
            eprintln!("   ✗ Failed to open port: {}", e);
            return;
        }
    };

    // Test 3: Check bytes_to_read (this is what bytes_available uses)
    println!("\n3. Testing bytes_to_read()...");
    for i in 1..=5 {
        match port.bytes_to_read() {
            Ok(bytes) => println!("   Attempt {}: {} bytes available", i, bytes),
            Err(e) => {
                eprintln!("   Attempt {}: Error - {}", i, e);
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(500));
    }

    // Test 4: Try to write data
    println!("\n4. Testing write (sending 'TEST\\n')...");
    match port.write_all(b"TEST\n") {
        Ok(_) => {
            match port.flush() {
                Ok(_) => println!("   ✓ Write successful"),
                Err(e) => eprintln!("   ✗ Flush failed: {}", e),
            }
        }
        Err(e) => eprintln!("   ✗ Write failed: {}", e),
    }

    // Test 5: Try to read data (with timeout)
    println!("\n5. Testing read (waiting 2 seconds for data)...");
    let mut buffer = [0u8; 256];
    match port.read(&mut buffer) {
        Ok(bytes_read) => {
            println!("   ✓ Read {} bytes", bytes_read);
            if bytes_read > 0 {
                let data = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("   Data: {:?}", data);
            }
        }
        Err(e) if e.kind() == io::ErrorKind::TimedOut => {
            println!("   ○ Read timeout (no data received, this is normal)");
        }
        Err(e) => eprintln!("   ✗ Read failed: {}", e),
    }

    // Test 6: Check bytes_to_read again
    println!("\n6. Testing bytes_to_read() again after I/O...");
    match port.bytes_to_read() {
        Ok(bytes) => println!("   ✓ {} bytes available", bytes),
        Err(e) => eprintln!("   ✗ Error: {}", e),
    }

    // Test 7: Clear buffer
    println!("\n7. Testing clear buffer...");
    match port.clear(serialport::ClearBuffer::All) {
        Ok(_) => println!("   ✓ Buffer cleared"),
        Err(e) => eprintln!("   ✗ Clear failed: {}", e),
    }

    println!("\n=== Test Complete ===");
    println!("\nNote: If bytes_to_read() is returning 0 or failing consistently,");
    println!("this indicates an issue with the serialport library or driver.");
}
