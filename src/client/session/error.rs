use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Connection(crate::client::connection::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Connection(e) => {
                write!(f, "Connection error: {e}")
            }
        }
    }
}
