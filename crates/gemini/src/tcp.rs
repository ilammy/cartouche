use std::{
    fmt, io,
    net::{SocketAddr, TcpStream, ToSocketAddrs},
};

pub fn connect(host: impl ToSocketAddrs + fmt::Debug) -> io::Result<TcpStream> {
    fn connect(addr: SocketAddr) -> io::Result<Option<TcpStream>> {
        loop {
            match TcpStream::connect(addr) {
                Ok(stream) => return Ok(Some(stream)),
                Err(err) => match err.kind() {
                    io::ErrorKind::Interrupted => continue,
                    io::ErrorKind::ConnectionRefused | io::ErrorKind::TimedOut => return Ok(None),
                    _ => return Err(err),
                },
            }
        }
    }
    for addr in host.to_socket_addrs()? {
        if let Some(stream) = connect(addr)? {
            return Ok(stream);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::ConnectionRefused,
        format!("host unreachable: {:?}", host),
    ))
}
