# Reverse-Shell

## Overview

The Reverse-Shell project is a demonstration of a reverse shell implemented in Rust. This tool establishes a connection from a target machine back to an attacker's machine, enabling remote command execution. It serves as an educational resource for understanding network programming, socket communication, and security concepts related to remote access and exploitation.

## Features

- **Cross-Platform Compatibility**: The reverse shell functions on both Windows and Unix-based systems (Linux, macOS).
- **Command Execution**: Executes commands received from the attacker and returns the output back to the attacker's console.
- **Minimal Dependencies**: Utilizes Rust's standard library for networking and process management, ensuring a lightweight footprint.

## Usage

1. **Set Up a Listener**: Use a tool like Netcat or Ncat on the attacker's machine to listen for incoming connections.
2. **Configure the Reverse Shell**: Modify the code to include the attacker's IP address and the port to which the connection should be established.
3. **Run the Shell**: Compile and execute the reverse shell on the target machine to initiate the connection.

## Getting Started

### Prerequisites

- Rust programming language installed on your machine.
- Basic knowledge of Rust and networking concepts.

### Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/b17w1z4rd/Reverse-Shell.git
   cd Reverse-Shell
2. Compile the project
   **rustc reverse_shell.rs**
3. Set up your listener and run the compiled binary.
