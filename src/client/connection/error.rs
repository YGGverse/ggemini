use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Closed,
    Tls(glib::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Closed => write!(f, "Socket connection closed"),
            Self::Tls(reason) => write!(f, "Could not create TLS connection: {reason}"),
        }
    }
}
