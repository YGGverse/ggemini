use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Decode(std::string::FromUtf8Error),
    Protocol,
    Undefined,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Decode(e) => {
                write!(f, "Decode error: {e}")
            }
            Self::Protocol => {
                write!(f, "Protocol error")
            }
            Self::Undefined => {
                write!(f, "MIME type undefined")
            }
        }
    }
}
