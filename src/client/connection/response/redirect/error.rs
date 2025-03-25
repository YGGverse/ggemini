use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    FirstByte(u8),
    Permanent(super::permanent::Error),
    SecondByte(u8),
    Temporary(super::temporary::Error),
    UndefinedFirstByte,
    UndefinedSecondByte,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::FirstByte(b) => {
                write!(f, "Unexpected first byte: {b}")
            }
            Self::Permanent(e) => {
                write!(f, "Permanent parse error: {e}")
            }
            Self::SecondByte(b) => {
                write!(f, "Unexpected second byte: {b}")
            }
            Self::Temporary(e) => {
                write!(f, "Temporary parse error: {e}")
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

/// Handle `super::uri` method
#[derive(Debug)]
pub enum UriError {
    BaseHost,
    ParseRelative(glib::Error),
}

impl Display for UriError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::BaseHost => {
                write!(f, "URI base host required")
            }
            Self::ParseRelative(e) => {
                write!(f, "URI parse relative error: {e}")
            }
        }
    }
}
