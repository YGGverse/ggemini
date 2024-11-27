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
            Self::Decode(reason) => {
                write!(f, "Decode error: {reason}")
            }
            Self::Protocol => {
                write!(f, "Protocol error")
            }
            Self::Undefined => {
                write!(f, "Undefined error")
            }
        }
    }
}
