use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Connect(glib::Error),
    Connectable(String),
    Connection(crate::client::connection::Error),
    NetworkAddress(crate::gio::network_address::Error),
    Request(glib::Error),
    Response(crate::client::response::Error),
    Write(glib::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Connectable(uri) => {
                write!(f, "Could not create connectable address for {uri}")
            }
            Self::Connection(reason) => {
                write!(f, "Connection error: {reason}")
            }
            Self::Connect(reason) => {
                write!(f, "Connect error: {reason}")
            }
            Self::NetworkAddress(reason) => {
                write!(f, "Network address error: {reason}")
            }
            Self::Request(reason) => {
                write!(f, "Request error: {reason}")
            }
            Self::Response(reason) => {
                write!(f, "Response error: {reason}")
            }
            Self::Write(reason) => {
                write!(f, "I/O Write error: {reason}")
            }
        }
    }
}
