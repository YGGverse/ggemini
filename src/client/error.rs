use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Connect(gio::NetworkAddress, glib::Error),
    Connection(gio::SocketConnection, crate::client::connection::Error),
    NetworkAddress(crate::client::connection::request::Error),
    Request(
        crate::client::connection::Connection,
        crate::client::connection::Error,
    ),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Connect(_, e) => {
                write!(f, "Connect error: {e}")
            }
            Self::Connection(_, e) => {
                write!(f, "Connection init error: {e}")
            }
            Self::NetworkAddress(e) => {
                write!(f, "Network address error: {e}")
            }
            Self::Request(_, e) => {
                write!(f, "Connection error: {e}")
            }
        }
    }
}
