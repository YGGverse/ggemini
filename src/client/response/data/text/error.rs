use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    BufferOverflow,
    Decode(std::string::FromUtf8Error),
    InputStream(glib::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::BufferOverflow => {
                write!(f, "Buffer overflow")
            }
            Self::Decode(reason) => {
                write!(f, "Decode error: {reason}")
            }
            Self::InputStream(reason) => {
                write!(f, "Input stream read error: {reason}")
            }
        }
    }
}
