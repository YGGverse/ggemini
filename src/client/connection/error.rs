use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Rehandshake(glib::Error),
    SocketConnection(glib::Error),
    SocketConnectionClosed,
    TlsClientConnection(glib::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Rehandshake(e) => {
                write!(f, "Rehandshake error: {e}")
            }
            Self::SocketConnectionClosed => write!(f, "Socket connection closed"),
            Self::SocketConnection(e) => {
                write!(f, "Socket connection error: {e}")
            }
            Self::TlsClientConnection(e) => {
                write!(f, "TLS client connection error: {e}")
            }
        }
    }
}
