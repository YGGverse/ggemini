use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    FirstByte(u8),
    NotAuthorized(super::not_authorized::Error),
    NotValid(super::not_valid::Error),
    Required(super::required::Error),
    SecondByte(u8),
    UndefinedFirstByte,
    UndefinedSecondByte,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::FirstByte(b) => {
                write!(f, "Unexpected first byte: {b}")
            }
            Self::NotAuthorized(e) => {
                write!(f, "NotAuthorized status parse error: {e}")
            }
            Self::NotValid(e) => {
                write!(f, "NotValid status parse error: {e}")
            }
            Self::Required(e) => {
                write!(f, "Required status parse error: {e}")
            }
            Self::SecondByte(b) => {
                write!(f, "Unexpected second byte: {b}")
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
