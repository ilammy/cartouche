use crate::status::Status;
use crate::tls;
use crate::tls::Stream;
use std::cmp::min;
use std::io;
use std::str;
use std::convert::TryInto;

pub struct Response {
    stream: Stream,
    state: State,
    buffer: Vec<u8>,
    status: Status,
    meta: String,
}

enum State {
    ReadingHeader,
    ReadingData,
    Complete,
}

impl Response {
    pub(crate) fn new(stream: Stream) -> Self {
        Self {
            stream,
            state: State::ReadingHeader,
            buffer: Vec::new(),
            status: Status::BadRequest,
            meta: String::new(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Protocol(#[from] ProtocolError),
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    TLS(rustls::TLSError),
    #[error("operation interrupted")]
    Interrupted,
    #[error("connection terminated")]
    Terminated,
}

#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    #[error("unexpected end of stream")]
    UnexpectedEndOfStream,
    #[error("header too short")]
    HeaderTooShort,
    #[error("header too long")]
    HeaderTooLong,
    #[error("malformed header")]
    HeaderMalformed,
    #[error("unknown status code")]
    UnknownStatus,
}

impl Response {
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        match self.state {
            State::ReadingHeader => {
                let (mut buffer, line_ending) = read_header(&mut self.stream)?;
                let (header, data) = (&buffer[..line_ending], &buffer[line_ending + 2..]);
                let (status, meta) = parse_header(header)?;
                self.status = status;
                self.meta = meta.to_owned();

                match status {
                    Status::Success => {
                        // TODO:
                    }
                    _ => unimplemented!("status {:?} is not supported yet", status),
                }

                if data.is_empty() {
                    self.state = State::ReadingData;
                    return self.read(buf);
                }

                let len = min(data.len(), buf.len());
                buf[..len].copy_from_slice(&data[..len]);
                self.buffer = buffer.split_off(len);
                self.state = State::ReadingData;
                Ok(len)
            }
            State::ReadingData => {
                if !self.buffer.is_empty() {
                    let len = min(self.buffer.len(), buf.len());
                    buf[..len].copy_from_slice(&self.buffer[..len]);
                    self.buffer = self.buffer.split_off(len);
                    return Ok(len);
                }
                match self.stream.read(buf) {
                    Ok(read) => Ok(read),
                    Err(tls::Error::Terminated) => {
                        self.state = State::Complete;
                        Err(Error::Terminated)
                    }
                    Err(other) => Err(other.into()),
                }
            }
            State::Complete => Err(Error::Terminated),
        }
    }
}

fn read_header(stream: &mut Stream) -> Result<(Vec<u8>, usize), Error> {
    let mut buffer = vec![0; 1024];
    let mut filled = 0usize;
    let line_ending = loop {
        if filled == buffer.len() {
            return Err(Error::Protocol(ProtocolError::HeaderTooLong));
        }
        // Compare one byte before, just in case the last one of the previous buffer
        // was \r and we're reading in \n that follows it just now.
        let before = filled.saturating_sub(1);
        match stream.read(&mut buffer[filled..]) {
            Ok(read) => filled += read,
            Err(tls::Error::Interrupted) => continue,
            Err(tls::Error::Terminated) => {
                return Err(Error::Protocol(ProtocolError::UnexpectedEndOfStream))
            }
            Err(other) => return Err(other.into()),
        }
        if let Some(index) = line_ending(&buffer[before..filled]) {
            break index;
        }
    };
    buffer.truncate(filled);
    Ok((buffer, line_ending))
}

fn line_ending(slice: &[u8]) -> Option<usize> {
    slice
        .windows(2)
        .position(|bytes| bytes == b"\r\n")
        .map(|index| index)
}

fn parse_header(header: &[u8]) -> Result<(Status, &str), ProtocolError> {
    if header.len() < 2 {
        return Err(ProtocolError::HeaderTooShort);
    }
    if header.len() > 2 && header[2] != b' ' {
        return Err(ProtocolError::HeaderMalformed);
    }
    let status = &header[0..2];
    let meta = if header.len() > 2 { &header[3..] } else { b"" };
    let status = str::from_utf8(status)
        .map_err(|_| ProtocolError::HeaderMalformed)?
        .parse::<u8>()
        .map_err(|_| ProtocolError::HeaderMalformed)?
        .try_into()
        .map_err(|_| ProtocolError::UnknownStatus)?;
    let meta = str::from_utf8(meta).map_err(|_| ProtocolError::HeaderMalformed)?;
    Ok((status, meta))
}

impl From<tls::Error> for Error {
    fn from(error: tls::Error) -> Self {
        match error {
            tls::Error::IO(error) => Self::IO(error),
            tls::Error::TLS(error) => Self::TLS(error),
            tls::Error::Interrupted => Self::Interrupted,
            tls::Error::Terminated => Self::Terminated,
        }
    }
}
