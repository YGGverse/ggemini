use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Request((Vec<u8>, glib::Error)),
    Response(crate::client::connection::response::Error),
    TlsClientConnection(glib::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Request((_, e)) => {
                write!(f, "Request error: {e}")
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
