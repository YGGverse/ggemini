use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Decode(std::string::FromUtf8Error),
    Protocol,
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
        }
    }
}
