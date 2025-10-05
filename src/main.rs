use std::{
    io::{BufRead, BufReader, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    str::FromStr,
};

fn main() {
    let raw_args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = raw_args.iter().map(|s| s.as_str()).collect();
    match args.as_slice() {
        ["client", addr] => match parse_and_run_client(addr) {
            Ok(()) => {}
            Err(e) => eprintln!("error: {e}"),
        },
        ["server"] => {
            if let Err(e) = run_server() {
                eprintln!("error: {e}");
            }
        }
        _ => print_usage(),
    }
}

/// Parse string to address and run client.
fn parse_and_run_client(raw_addr: &str) -> std::io::Result<()> {
    let addr = SocketAddr::from_str(raw_addr).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "invalid address format, expected IP:PORT",
        )
    })?;

    run_client(addr)
}

/// Wait for connection with client,
/// receive and send message.
fn run_server() -> std::io::Result<()> {
    // Init connection
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr)?;
    println!(
        "Waiting for connection on {:?}...",
        listener.local_addr().unwrap()
    );
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
fn run_client(addr: SocketAddr) -> std::io::Result<()> {
    // Init connection
    println!("Connecting to server...");
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
