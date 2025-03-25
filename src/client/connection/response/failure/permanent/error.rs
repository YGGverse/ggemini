use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    BadRequest(super::bad_request::Error),
    Default(super::default::Error),
    FirstByte(u8),
    Gone(super::gone::Error),
    NotFound(super::not_found::Error),
    ProxyRequestRefused(super::proxy_request_refused::Error),
    SecondByte(u8),
    UndefinedFirstByte,
    UndefinedSecondByte,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::BadRequest(e) => {
                write!(f, "BadRequest parse error: {e}")
            }
            Self::Default(e) => {
                write!(f, "Default parse error: {e}")
            }
            Self::FirstByte(b) => {
                write!(f, "Unexpected first byte: {b}")
            }
            Self::Gone(e) => {
                write!(f, "Gone parse error: {e}")
            }
            Self::NotFound(e) => {
                write!(f, "NotFound parse error: {e}")
            }
            Self::ProxyRequestRefused(e) => {
                write!(f, "ProxyRequestRefused parse error: {e}")
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
