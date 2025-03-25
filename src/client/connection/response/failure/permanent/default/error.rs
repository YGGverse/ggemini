use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Code,
    Header(crate::client::connection::response::HeaderBytesError),
    Utf8Error(std::str::Utf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Code => {
                write!(f, "Unexpected status code")
            }
            Self::Header(e) => {
                write!(f, "Header error: {e}")
            }
            Self::Utf8Error(e) => {
                write!(f, "UTF-8 decode error: {e}")
            }
        }
    }
}
