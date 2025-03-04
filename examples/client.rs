use slowcat::*;

use std::io;
use std::mem;
use std::net::Ipv4Addr;

use clap::Parser;

const RX_BUFFER_SIZE: usize = 8192;
type RxBuffer = [u8; RX_BUFFER_SIZE];

const REQUEST: &str = "PING\r\n\r\n";

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    ip: Ipv4Addr,
    #[clap(short, long)]
    port: u16,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    let remote_addr = args.ip.to_bits();
    let remote_port = args.port;

    // 1
    let sockfd = socket(AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0)?;
    #[cfg(target_vendor = "apple")]
    // By default write() to a closed socket sends a signal. Setting
    // SO_NOSIGPIPE throws EPIPE error instead which is easy to handle
    setsockopt(sockfd, libc::SOL_SOCKET, libc::SO_NOSIGPIPE, 1)?;

    // 2
    connect(
        sockfd,
        sockaddr_in {
            sin_family: AF_INET /* IPv4 */ as libc::sa_family_t,
            // NOTE: s_addr & sin_port are in network byte order (big endian)
            sin_addr: in_addr {
                s_addr: remote_addr.to_be() as libc::in_addr_t,
            },
            sin_port: remote_port.to_be() as libc::in_port_t,
            ..unsafe { mem::zeroed() }
        },
    )?;

    // 3
    let tx_bytes = write(sockfd, REQUEST.as_bytes())?;
    dbg!(tx_bytes);
    println!("<- {REQUEST:?}");

    // 4
    let mut buffer: RxBuffer = [0; RX_BUFFER_SIZE];
    let rx_bytes = read(sockfd, &mut buffer)?;
    let rx_msg = std::str::from_utf8(&buffer[0..rx_bytes]).unwrap_or("");
    dbg!(rx_bytes);
    println!("-> {rx_msg:?}");

    // 5
    close(sockfd)?;

    Ok(())
}
