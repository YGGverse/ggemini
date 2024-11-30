use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Cancel,
    Closed,
    Rehandshake(glib::Error),
    SocketConnection(glib::Error),
    TlsClientConnection(glib::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Cancel => write!(f, "Cancellable not found"),
            Self::Closed => write!(f, "Connection closed"),
            Self::Rehandshake(e) => {
                write!(f, "Rehandshake error: {e}")
            }
            Self::SocketConnection(e) => {
                write!(f, "Socket connection error: {e}")
            }
            Self::TlsClientConnection(e) => {
                write!(f, "TLS client connection error: {e}")
            }
        }
    }
}
