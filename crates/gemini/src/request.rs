use crate::response::Response;
use crate::tls;
use crate::tls::Stream;
use rustls::ClientConfig;
use std::sync::Arc;
use std::io;
use url::Url;

pub struct Request;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    URL(#[from] url::ParseError),
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    TLS(rustls::TLSError),
}

const DEFAULT_GEMINI_PORT: u16 = 1965;

impl Request {
    pub fn perform(url: &str, config: &Arc<ClientConfig>) -> Result<Response, Error> {
        let url = Url::parse(url)?;
        let host = url.host_str().ok_or(url::ParseError::EmptyHost)?;
        let port = url.port().unwrap_or(DEFAULT_GEMINI_PORT);
        let mut stream = Stream::new(host, port, config)?;
        stream.establish_connection()?;
        stream.write(url.as_str().as_bytes())?;
        stream.write(b"\r\n")?;
        stream.flush()?;
        Ok(Response::new(stream))
    }
}

impl From<tls::Error> for Error {
    fn from(error: tls::Error) -> Self {
        match error {
            tls::Error::IO(error) => Self::IO(error),
            tls::Error::TLS(error) => Self::TLS(error),
            tls::Error::Interrupted | tls::Error::Terminated => {
                unreachable!("error must be handled before conversion")
            }
        }
    }
}
