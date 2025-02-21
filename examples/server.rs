use slowcat::*;

use std::net::Ipv4Addr;
use std::{io, mem};

use clap::Parser;

const LISTEN_BACKLOG: i32 = 5;
const RX_BUFFER_SIZE: usize = 256;
type RxBuffer = [u8; RX_BUFFER_SIZE];

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
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

    // 1. socket()
    let listen_sockfd = socket(AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0)?;
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_vendor = "apple"
    ))]
    // By default write() to a closed socket sends a signal. Setting
    // SO_NOSIGPIPE throws EPIPE error instead which is easy to handle
    setsockopt(listen_sockfd, libc::SOL_SOCKET, libc::SO_NOSIGPIPE, 1)?;

    // 2. bind()
    #[cfg(not(windows))]
    setsockopt(listen_sockfd, libc::SOL_SOCKET, libc::SO_REUSEADDR, 1)?;
    bind(
        listen_sockfd,
        sockaddr_in {
            #[cfg(any(
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_vendor = "apple"
            ))]
            sin_len: 0,

            sin_family: AF_INET as libc::sa_family_t, // IPv4
            // NOTE: s_addr & sin_port are in network byte order (big endian)
            sin_addr: in_addr {
                s_addr: listen_addr.to_be() as libc::in_addr_t,
            },
            sin_port: listen_port.to_be() as libc::in_port_t,
            sin_zero: [0; 8],
        },
    )?;

    // 3. listen()
    listen(listen_sockfd, LISTEN_BACKLOG)?;

    // 4. accept()
    let client_sockfd = accept(listen_sockfd)?;

    // 5. read()
    let mut buffer: RxBuffer = unsafe { mem::zeroed() };
    let rx_bytes = read(client_sockfd, &mut buffer)?;
    let s = std::str::from_utf8(&buffer).unwrap_or("");
    println!("[rx_len={rx_bytes}] <- {s}");

    // 6. write()
    let message = "PONG";
    let tx_bytes = write(client_sockfd, message.as_bytes())?;
    println!("[tx_len={tx_bytes}] -> {message}");

    // 7. close()
    close(client_sockfd)?;

    // 8. close()
    close(listen_sockfd)?;

    Ok(())
}
