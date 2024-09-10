use std::net::{TcpStream, SocketAddr};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use std::env;

fn scan_port(ip: &str, port: u16) -> bool {
    let socket_address = format!("{}:{}", ip, port);
    let socket_address: SocketAddr = socket_address.parse().expect("Invalid IP or port");

    TcpStream::connect_timeout(&socket_address, Duration::from_secs(1)).is_ok()
}

fn scan_ports(ip: &str, start_port: u16, end_port: u16) {
    let (tx, rx) = channel();
    println!("Scanning {} from port {} to {}...", ip, start_port, end_port);

    for port in start_port..=end_port {
        let tx = tx.clone();
        let ip = ip.to_string();

        thread::spawn(move || {
            if scan_port(&ip, port) {
                tx.send(Some(port)).expect("Failed to send the result");
            } else {
                tx.send(None).expect("Failed to send the result");
            }
        });
    }

    drop(tx);  // Close the sending end of the channel.

    for open_port in rx.iter() {
        if let Some(port) = open_port {
            println!("Port {} is open.", port);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: {} <IP> <start_port> <end_port>", args[0]);
        std::process::exit(1);
    }

    let ip = &args[1];
    let start_port: u16 = args[2].parse().expect("Invalid start port");
    let end_port: u16 = args[3].parse().expect("Invalid end port");

    scan_ports(ip, start_port, end_port);
}
