use crate::tcp;
use rustls::{ClientConfig, ClientSession, Session, TLSError};
use std::io;
use std::io::{Read, Write};
use std::{net::TcpStream, sync::Arc};
use webpki::DNSNameRef;

#[derive(Debug)]
pub struct Stream {
    stream: TcpStream,
    session: ClientSession,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("operation interrupted")]
    Interrupted,
    #[error("connection terminated")]
    Terminated,
    #[error(transparent)]
    IO(io::Error),
    #[error(transparent)]
    TLS(#[from] TLSError),
}

impl Stream {
    pub fn new(host: &str, port: u16, config: &Arc<ClientConfig>) -> io::Result<Self> {
        let stream = tcp::connect((host, port))?;
        let hostname = DNSNameRef::try_from_ascii_str(host).expect("hostname must be valid");
        let session = ClientSession::new(config, hostname);
        Ok(Self { stream, session })
    }

    fn complete_io(&mut self) -> Result<(), Error> {
        if self.session.wants_write() {
            self.session.write_tls(&mut self.stream)?;
        }
        if self.session.wants_read() {
            self.session.read_tls(&mut self.stream)?;
            self.session.process_new_packets()?;
        }
        Ok(())
    }

    pub fn establish_connection(&mut self) -> Result<(), Error> {
        while self.session.is_handshaking() {
            self.complete_io()?;
        }
        Ok(())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.complete_io()?;
        Ok(self.session.read(buf)?)
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        Ok(self.session.write(buf)?)
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        self.complete_io()?;
        self.session.flush()?;
        self.complete_io()?;
        Ok(())
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::Interrupted => Self::Interrupted,
            io::ErrorKind::ConnectionAborted => Self::Terminated,
            _ => Self::IO(error),
        }
    }
}

