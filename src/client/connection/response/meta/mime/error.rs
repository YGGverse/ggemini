use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Decode(std::string::FromUtf8Error),
    Protocol,
    Undefined(Option<String>),
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
            Self::Undefined(e) => {
                write!(
                    f,
                    "{}",
                    match e {
                        Some(value) => format!("`{value}` undefined"),
                        None => "Could not parse value".to_string(),
                    }
                )
            }
        }
    }
}
