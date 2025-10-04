# Rust TCP Demo
![GitHub License](https://img.shields.io/github/license/sebastian-j-ibanez/rust_tcp_demo?color=orange)

🦀 A client-server TCP demo in Rust. 🦀

## Instructions
Demo requires `rust`, `cargo`, `docker`, `docker compose`.

1. Clone the repository.
```
git clone https://github.com/sebastian-j-ibanez/rust_tcp_demo
```
2. Build and run containers.
```
docker compose up -d
```
3. View container logs.
```
docker compose logs
```

## Expected logs
```
server  | Waiting for connection on 0.0.0.0:8080...
server  | Connected to client...
server  | Received message: Hello, this is the client!
server  |
client  | Connecting to server...
client  | Connected...
client  | Received message: Hello, this is the server
client  |
```

## Usage

```
A Rust server-client TCP demo

Usage:
        rust_tcp_demo [command]

Commands:
        client  Start client.
        help    Print help.
        server  Start server.

```
