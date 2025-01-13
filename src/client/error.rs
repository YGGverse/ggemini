use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Connect(glib::Error),
    Connection(crate::client::connection::Error),
    Request(crate::client::connection::request::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Connection(e) => {
                write!(f, "Connection error: {e}")
            }
            Self::Connect(e) => {
                write!(f, "Connect error: {e}")
            }
            Self::Request(e) => {
                write!(f, "Request error: {e}")
            }
        }
    }
}
