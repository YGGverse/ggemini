use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    SocketConnectionClosed,
    TlsClientConnection(glib::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::SocketConnectionClosed => write!(f, "Socket connection closed"),
            Self::TlsClientConnection(reason) => {
                write!(f, "Could not create TLS connection: {reason}")
            }
        }
    }
}
