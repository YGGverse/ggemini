use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Response(crate::client::connection::response::Error),
    Stream(glib::Error),
    TlsClientConnection(glib::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Stream(e) => {
                write!(f, "TLS client connection error: {e}")
            }
            Self::Response(e) => {
                write!(f, "Response error: {e}")
            }
            Self::TlsClientConnection(e) => {
                write!(f, "TLS client connection error: {e}")
            }
        }
    }
}
