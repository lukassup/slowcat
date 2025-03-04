use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::{
    io::{self, BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

use clap::Parser;

const RX_BUFFER: usize = 8192; // Rust default on most platforms
const TX_BUFFER: usize = 8192; // Rust default on most platforms
const REQUEST: &str = "PING";
const RESPONSE: &str = "PONG\r\n";

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    ip: Ipv4Addr,
    #[clap(short, long)]
    port: u16,
}
fn main() -> io::Result<()> {
    let args = Args::parse();
    let addr = args.ip;
    let port = args.port;
    let sock_addr = SocketAddr::new(std::net::IpAddr::V4(addr), port);

    let listener = TcpListener::bind(sock_addr)?;
    println!("listening: tcp://{addr}:{port}");

    for stream in listener.incoming().map_while(Result::ok) {
        let peer = stream.peer_addr()?;
        let remote_addr = peer.ip();
        let remote_port = peer.port();
        println!("accepted: tcp://{remote_addr}:{remote_port}");
        handle_connection(stream)?;
        println!("closing: tcp://{remote_addr}:{remote_port}");
    }
    Ok(())
}

fn handle_connection(stream: TcpStream) -> io::Result<()> {
    let buf_reader = BufReader::with_capacity(RX_BUFFER, &stream);
    let request: Vec<_> = buf_reader
        .lines()
        .map_while(Result::ok)
        .take_while(|line| !line.is_empty())
        .collect();
    dbg!(&request);

    let mut buf_writer = BufWriter::with_capacity(TX_BUFFER, &stream);
    match request.first().map(String::as_str) {
        Some(REQUEST) => buf_writer.write_all(RESPONSE.as_bytes())?,
        _ => {}
    };

    Ok(())
}
