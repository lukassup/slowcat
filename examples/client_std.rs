use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::{
    io::{self, BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
};

use clap::Parser;

const RX_BUFFER: usize = 8192; // Rust default on most platforms
const TX_BUFFER: usize = 8192; // Rust default on most platforms
const REQUEST: &str = "PING\r\n";

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
    let stream = TcpStream::connect(sock_addr)?;
    println!("connected: tcp://{addr}:{port}");
    handle_connection(stream)
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buf_writer = BufWriter::with_capacity(TX_BUFFER, &mut stream);
    buf_writer.write_all(REQUEST_GET.as_bytes())?;
    drop(buf_writer);

    let buf_reader = BufReader::with_capacity(RX_BUFFER, &stream);
    let response: Vec<_> = buf_reader
        .lines()
        .map_while(Result::ok)
        .take_while(|line| !line.is_empty())
        .collect();
    dbg!(&response);
    Ok(())
}
