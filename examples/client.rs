use slowcat::*;
use std::{io, mem};

const RX_BUFFER_SIZE: usize = 256;
type RxBuffer = [u8; RX_BUFFER_SIZE];

use clap::Parser;
use std::net::Ipv4Addr;

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
    let remote_addr = args.ip.to_bits();
    let remote_port = args.port;

    // 1. socket()
    let sockfd = socket(AF_INET, SOCK_STREAM, 0)?;

    // 2. connect()
    connect(
        sockfd,
        sockaddr_in {
            #[cfg(any(
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "ios",
                target_os = "macos",
                target_os = "netbsd",
                target_os = "openbsd",
            ))]
            sin_len: 0,

            sin_family: AF_INET as libc::sa_family_t, // IPv4
            // NOTE: s_addr & sin_port are in network byte order (big endian)
            sin_addr: in_addr {
                s_addr: remote_addr.to_be() as libc::in_addr_t,
            },
            sin_port: remote_port.to_be() as libc::in_port_t,
            sin_zero: [0; 8],
        },
    )?;

    // 3. write()
    let message = "PING";
    let tx_bytes = write(sockfd, message.as_bytes())?;
    println!("[tx_len={tx_bytes}] -> {message}");

    // 4. read()
    let mut buffer: RxBuffer = unsafe { mem::zeroed() };
    let rx_bytes = read(sockfd, &mut buffer)?;
    let s = std::str::from_utf8(&buffer).unwrap_or("");
    println!("[rx_len={rx_bytes}] <- {s}");

    // 5. close()
    close(sockfd)?;

    Ok(())
}
