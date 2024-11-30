use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Connect(glib::Error),
    Connectable(String),
    Connection(crate::client::connection::Error),
    NetworkAddress(crate::gio::network_address::Error),
    OutputStream(glib::Error),
    Request(glib::Error),
    Response(crate::client::response::Error),
    Session(crate::client::session::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Connectable(uri) => {
                write!(f, "Could not create connectable address for {uri}")
            }
            Self::Connection(e) => {
                write!(f, "Connection error: {e}")
            }
            Self::Connect(e) => {
                write!(f, "Connect error: {e}")
            }
            Self::NetworkAddress(e) => {
                write!(f, "Network address error: {e}")
            }
            Self::OutputStream(e) => {
                write!(f, "Output stream error: {e}")
            }
            Self::Request(e) => {
                write!(f, "Request error: {e}")
            }
            Self::Response(e) => {
                write!(f, "Response error: {e}")
            }
            Self::Session(e) => {
                write!(f, "Session error: {e}")
            }
        }
    }
}
