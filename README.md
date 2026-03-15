# TCP Server and Client Documentation

## Overview
This repository contains a TCP server and client implemented in Rust. The server is designed to handle multiple client connections and provides functionalities for data exchange over TCP.

## Usage
1. Clone the repository:
   ```bash
   git clone https://github.com/mekem688/serveur_tcp_rust-
   ```
2. Navigate to the project directory:
   ```bash
   cd serveur_tcp_rust-
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run the TCP server:
   ```bash
   cargo run --bin server
   ```
5. Run the TCP client:
   ```bash
   cargo run --bin client
   ```

## Port 8000 Error Fix
If you encounter errors related to port 8000, ensure that:
- The port is not already in use by another application. You can check this using:
  ```bash
  lsof -i :8000
  ```
- If the port is occupied, either kill the process using it or configure the server to listen on a different port by modifying the source code.

## Troubleshooting
- Make sure to handle any network issues or firewall settings that may obstruct the TCP communication.
- If you face problems with connection limits, consider optimizing your server settings to accommodate more clients.

## Conclusion
This documentation serves as a guide to setup and run the TCP server and client. For more detailed information, refer to the source code and comments within the application.