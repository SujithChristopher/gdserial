// Test specifically for bytes_to_read issue
// Usage: cargo run --example bytes_to_read_test COM10

use std::env;
use std::time::Duration;

fn main() {
    let port_name = env::args().nth(1).expect("Usage: program <PORT>");

    println!("Opening {}...", port_name);

    let mut port = serialport::new(&port_name, 9600)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open port");

    println!("Port opened successfully\n");
    println!("Calling bytes_to_read() 20 times with 500ms delay...\n");

    for i in 1..=20 {
        match port.bytes_to_read() {
            Ok(bytes) => {
                println!("Call {}: bytes_to_read() = {}", i, bytes);

                // If bytes available, try to read them
                if bytes > 0 {
                    let mut buffer = vec![0u8; bytes as usize];
                    match std::io::Read::read(&mut port, &mut buffer) {
                        Ok(n) => {
                            let text = String::from_utf8_lossy(&buffer[..n]);
                            println!("       Read {} bytes: {:?}", n, text);
                        }
                        Err(e) => println!("       Read error: {}", e),
                    }
                }
            }
            Err(e) => {
                println!("Call {}: ERROR - {}", i, e);
                println!("       Error kind: {:?}", e.kind());
                break;
            }
        }

        std::thread::sleep(Duration::from_millis(500));
    }

    println!("\nTest complete");
}
