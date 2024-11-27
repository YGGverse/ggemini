use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Connectable(String),
    Connection(super::connection::Error),
    Connect(glib::Error),
    Request(glib::Error),
    Response(super::response::Error),
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
