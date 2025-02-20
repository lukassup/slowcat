/**
 * Some primitive Rust wrappers for libc calls
 */

pub use libc::AF_INET;
pub use libc::INADDR_LOOPBACK;
pub use libc::SOCK_STREAM;
pub use libc::in_addr;
pub use libc::sockaddr_in;

use std::io;

use libc::c_int;
use libc::c_void;
use libc::ssize_t;

/// man 2 errno
pub fn errno() -> io::Error {
    io::Error::last_os_error()
}

/// man 2 socket
pub fn socket(domain: c_int, ty: c_int, protocol: c_int) -> Result<c_int, io::Error> {
    println!("socket()");
    let sockfd = unsafe { libc::socket(domain, ty, protocol) };
    if sockfd < 0 {
        return Err(errno());
    }
    Ok(sockfd)
}

/// man 2 connect
pub fn connect(socket: c_int, address: libc::sockaddr_in) -> Result<(), io::Error> {
    println!("connect()");
    let retval = unsafe {
        libc::connect(
            socket,
            (&address as *const libc::sockaddr_in).cast(),
            size_of::<libc::sockaddr_in>().try_into().unwrap(),
        )
    };
    if retval < 0 {
        return Err(errno());
    }
    Ok(())
}

/// man 2 bind
pub fn bind(socket: c_int, address: libc::sockaddr_in) -> Result<(), io::Error> {
    println!("bind()");
    let retval = unsafe {
        libc::bind(
            socket,
            (&address as *const libc::sockaddr_in).cast(),
            size_of::<libc::sockaddr_in>().try_into().unwrap(),
        )
    };
    if retval < 0 {
        return Err(errno());
    }
    Ok(())
}

/// man 2 listen
pub fn listen(socket: c_int, backlog: c_int) -> Result<(), io::Error> {
    println!("listen()");
    if unsafe { libc::listen(socket, backlog) } < 0 {
        return Err(errno());
    }
    Ok(())
}

/// man 2 accept
pub fn accept(socket: c_int) -> Result<c_int, io::Error> {
    println!("accept()");
    let addr: *mut libc::sockaddr = std::ptr::null_mut();
    let alen: *mut u32 = std::ptr::null_mut();
    let sockfd = unsafe { libc::accept(socket, addr, alen) };
    if sockfd < 0 {
        return Err(errno());
    }
    Ok(sockfd)
}

/// man 2 read
pub fn read(socket: c_int, buffer: &mut [u8]) -> Result<ssize_t, io::Error> {
    println!("read()");
    let rx_bytes = unsafe { libc::read(socket, buffer.as_mut_ptr() as *mut c_void, buffer.len()) };
    if rx_bytes < 0 {
        return Err(errno());
    }
    Ok(rx_bytes)
}

/// man 2 write
pub fn write(socket: c_int, buffer: &[u8]) -> Result<ssize_t, io::Error> {
    println!("write()");
    let tx_bytes = unsafe { libc::write(socket, buffer.as_ptr() as *const c_void, buffer.len()) };
    if tx_bytes < 0 {
        return Err(errno());
    }
    Ok(tx_bytes)
}

/// man 2 close
pub fn close(fd: c_int) -> Result<(), io::Error> {
    println!("close()");
    if unsafe { libc::close(fd) } < 0 {
        return Err(errno());
    }
    Ok(())
}
