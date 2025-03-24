use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Default(super::default::Error),
    FirstByte(u8),
    Protocol,
    SecondByte(u8),
    Sensitive(super::sensitive::Error),
    UndefinedFirstByte,
    UndefinedSecondByte,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Default(e) => {
                write!(f, "Default parse error: {e}")
            }
            Self::FirstByte(b) => {
                write!(f, "Unexpected first byte: {b}")
            }
            Self::Protocol => {
                write!(f, "Protocol error")
            }
            Self::SecondByte(b) => {
                write!(f, "Unexpected second byte: {b}")
            }
            Self::Sensitive(e) => {
                write!(f, "Sensitive parse error: {e}")
            }
            Self::UndefinedFirstByte => {
                write!(f, "Undefined first byte")
            }
            Self::UndefinedSecondByte => {
                write!(f, "Undefined second byte")
            }
        }
    }
}
