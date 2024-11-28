use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    SocketConnectionClosed,
    SocketConnection(glib::Error),
    TlsClientConnection(glib::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::SocketConnectionClosed => write!(f, "Socket connection closed"),
            Self::SocketConnection(reason) => {
                write!(f, "Socket connection error: {reason}")
            }
            Self::TlsClientConnection(reason) => {
                write!(f, "TLS client connection error: {reason}")
            }
        }
    }
}
