/**
 * Some primitive Rust wrappers for libc calls
 */
use std::io;
use std::mem;

pub use libc::AF_INET;
pub use libc::INADDR_LOOPBACK;
pub use libc::SOCK_STREAM;
pub use libc::c_int;
pub use libc::c_void;
pub use libc::in_addr;
pub use libc::sockaddr_in;
pub use libc::socklen_t;
pub use libc::ssize_t;

#[cfg(not(target_vendor = "apple"))]
pub const SOCK_CLOEXEC: c_int = libc::SOCK_CLOEXEC;
#[cfg(target_vendor = "apple")]
pub const SOCK_CLOEXEC: c_int = 0;

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

/// man 2 setsockopt
pub fn setsockopt<T>(
    socket: c_int,
    level: c_int,
    option_name: c_int,
    option_value: T,
) -> Result<c_int, io::Error> {
    println!("setsockopt()");
    let sockfd = unsafe {
        libc::setsockopt(
            socket,
            level,
            option_name,
            (&raw const option_value) as *const _,
            mem::size_of::<T>() as socklen_t,
        )
    };
    if sockfd < 0 {
        return Err(errno());
    }
    Ok(sockfd)
}

/// man 2 connect
pub fn connect<T>(socket: c_int, address: T) -> Result<(), io::Error> {
    println!("connect()");
    let retval = unsafe {
        libc::connect(
            socket,
            (&address as *const T).cast(),
            size_of::<T>().try_into().unwrap(),
        )
    };
    if retval < 0 {
        return Err(errno());
    }
    Ok(())
}

/// man 2 bind
pub fn bind<T>(socket: c_int, address: T) -> Result<(), io::Error> {
    println!("bind()");
    let retval = unsafe {
        libc::bind(
            socket,
            (&address as *const T).cast(),
            size_of::<T>().try_into().unwrap(),
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
///
/// man 2 accept4
pub fn accept(socket: c_int) -> Result<c_int, io::Error> {
    let addr: *mut libc::sockaddr = std::ptr::null_mut();
    let alen: *mut u32 = std::ptr::null_mut();

    cfg_if::cfg_if! {
        if #[cfg(not(target_vendor = "apple"))] {
            println!("accept4()");
            let sockfd = unsafe { libc::accept4(socket, addr, alen, SOCK_CLOEXEC) };
        } else  {
            println!("accept()");
            let sockfd = unsafe { libc::accept(socket, addr, alen) };
        }
    }

    if sockfd < 0 {
        return Err(errno());
    }
    Ok(sockfd)
}

/// man 2 read
pub fn read(socket: c_int, buffer: &mut [u8]) -> Result<usize, io::Error> {
    println!("read()");
    let rx_bytes = unsafe { libc::read(socket, buffer.as_mut_ptr() as *mut c_void, buffer.len()) };
    if rx_bytes < 0 {
        return Err(errno());
    }
    Ok(rx_bytes as usize)
}

/// man 2 write
pub fn write(socket: c_int, buffer: &[u8]) -> Result<usize, io::Error> {
    println!("write()");
    let tx_bytes = unsafe { libc::write(socket, buffer.as_ptr() as *const c_void, buffer.len()) };
    if tx_bytes < 0 {
        return Err(errno());
    }
    Ok(tx_bytes as usize)
}

/// man 2 close
pub fn close(fd: c_int) -> Result<(), io::Error> {
    println!("close()");
    if unsafe { libc::close(fd) } < 0 {
        return Err(errno());
    }
    Ok(())
}
