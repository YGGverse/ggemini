use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Connect(glib::Error),
    Connection(gio::SocketConnection, crate::client::connection::Error),
    NetworkAddress(crate::client::connection::request::Error),
    Request(crate::client::connection::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Connect(e) => {
                write!(f, "Connect error: {e}")
            }
            Self::Connection(_, e) => {
                write!(f, "Connection init error: {e}")
            }
            Self::NetworkAddress(e) => {
                write!(f, "Network address error: {e}")
            }
            Self::Request(e) => {
                write!(f, "Connection error: {e}")
            }
        }
    }
}
