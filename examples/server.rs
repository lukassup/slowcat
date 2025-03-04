use slowcat::*;

use std::io;
use std::mem;
use std::net::Ipv4Addr;

use clap::Parser;

const LISTEN_BACKLOG: i32 = 5;
const RX_BUFFER_SIZE: usize = 8192;
type RxBuffer = [u8; RX_BUFFER_SIZE];

const RESPONSE: &str = "PONG\r\n";

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    ip: Ipv4Addr,
    #[clap(short, long)]
    port: u16,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    let listen_addr = args.ip.to_bits();
    let listen_port = args.port;

    // 1
    let listen_sockfd = socket(AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0)?;
    #[cfg(target_vendor = "apple")]
    // By default write() to a closed socket sends a signal. Setting
    // SO_NOSIGPIPE throws EPIPE error instead which is easy to handle
    setsockopt(listen_sockfd, libc::SOL_SOCKET, libc::SO_NOSIGPIPE, 1)?;
    #[cfg(not(windows))]
    setsockopt(listen_sockfd, libc::SOL_SOCKET, libc::SO_REUSEADDR, 1)?;

    // 2
    bind(
        listen_sockfd,
        sockaddr_in {
            sin_family: AF_INET /* IPv4 */ as libc::sa_family_t,
            // NOTE: s_addr & sin_port are in network byte order (big endian)
            sin_addr: in_addr {
                s_addr: listen_addr.to_be() as libc::in_addr_t,
            },
            sin_port: listen_port.to_be() as libc::in_port_t,
            ..unsafe { mem::zeroed() }
        },
    )?;

    // 3
    listen(listen_sockfd, LISTEN_BACKLOG)?;

    // 4
    while let Ok(client_sockfd) = accept(listen_sockfd) {
        // 5
        let mut rx_data: Vec<u8> = Vec::with_capacity(RX_BUFFER_SIZE);
        loop {
            let mut rx_buf: RxBuffer = [0; RX_BUFFER_SIZE];
            let rx_bytes = read(client_sockfd, &mut rx_buf)? as usize;
            if rx_bytes < RX_BUFFER_SIZE {
                break;
            }
            rx_data.extend(&rx_buf[0..rx_bytes]);
            println!("<- [rx_bytes={rx_bytes}]");
        }
        let rx_msg = std::str::from_utf8(&rx_data).unwrap_or("");
        println!("<- {rx_msg:?}");

        // 6
        let tx_bytes = write(client_sockfd, RESPONSE.as_bytes())?;
        println!("-> [tx_bytes={tx_bytes}]");
        println!("-> {RESPONSE:?}");

        // 7
        close(client_sockfd)?;
    }

    // 8, TODO: cleanup listen socket after receiving signal
    close(listen_sockfd)?;

    Ok(())
}
