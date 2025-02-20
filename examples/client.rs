use std::{io, mem};
use slowcat::*;

const REMOTE_ADDR: u32 = INADDR_LOOPBACK; // 127.0.0.1
const REMOTE_PORT: u16 = 8080;
const RX_BUFFER_SIZE: usize = 256;

type RxBuffer = [u8; RX_BUFFER_SIZE];

fn main() -> Result<(), io::Error> {
    // 1. socket()
    let sockfd = socket(AF_INET, SOCK_STREAM, 0)?;

    // 2. connect()
    connect(
        sockfd,
        sockaddr_in {
            sin_family: AF_INET as u16, // IPv4
            // NOTE: s_addr & sin_port are in network byte order (big endian)
            sin_addr: in_addr {
                s_addr: REMOTE_ADDR.to_be(),
            },
            sin_port: REMOTE_PORT.to_be(),
            sin_zero: [0u8; 8],
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
