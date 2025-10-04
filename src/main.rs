use std::{
    io::{BufRead, BufReader, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("client") => {
            if let Err(e) = run_client() {
                eprintln!("error: {}", e);
            };
        }
        Some("server") => {
            if let Err(e) = run_server() {
                eprintln!("error: {}", e);
            };
        }
        _ => print_usage(),
    }
}

const DEFAULT_ADDR: ([u8; 4], u16) = ([127, 0, 0, 1], 8080);

/// Wait for connection with client,
/// receive and send message.
fn run_server() -> std::io::Result<()> {
    // Init connection
    let addr = SocketAddr::from(DEFAULT_ADDR);
    let listener = TcpListener::bind(addr)?;
    println!("Waiting for connection...");
    let (mut stream, _) = listener.accept()?;
    println!("Connected to client...");

    // Read message
    let mut reader = BufReader::new(&stream);
    let mut rx_buf = String::new();
    reader.read_line(&mut rx_buf)?;
    println!("Received message: {}", rx_buf);

    // Send message
    let tx_buf = "Hello, this is the server\n".as_bytes();
    stream.write_all(tx_buf)?;

    Ok(())
}

/// Establish connection with server,
/// send and receive message.
fn run_client() -> std::io::Result<()> {
    // Init connection
    println!("Connecting to server...");
    let addr = SocketAddr::from(DEFAULT_ADDR);
    let mut stream = TcpStream::connect(addr)?;
    println!("Connected...");

    // Send message
    let mut tx_buf = "Hello, this is the client!\n".as_bytes();
    stream.write_all(&mut tx_buf)?;

    // Read message
    let mut reader = BufReader::new(&stream);
    let mut rx_buf = String::new();
    reader.read_line(&mut rx_buf)?;
    println!("Received message: {}", rx_buf);

    Ok(())
}

/// Print usage.
fn print_usage() {
    println!("A Rust server-client TCP demo\n");
    println!("Usage:\n\trust_tcp_demo [command]\n");
    println!("Commands:");
    println!("\tclient\tStart client.");
    println!("\thelp\tPrint help.");
    println!("\tserver\tStart server.");
}
