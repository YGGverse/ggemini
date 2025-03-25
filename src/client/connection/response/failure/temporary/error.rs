use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    CgiError(super::cgi_error::Error),
    Default(super::default::Error),
    FirstByte(u8),
    ProxyError(super::proxy_error::Error),
    SecondByte(u8),
    ServerUnavailable(super::server_unavailable::Error),
    SlowDown(super::slow_down::Error),
    UndefinedFirstByte,
    UndefinedSecondByte,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::CgiError(e) => {
                write!(f, "CgiError parse error: {e}")
            }
            Self::Default(e) => {
                write!(f, "Default parse error: {e}")
            }
            Self::FirstByte(b) => {
                write!(f, "Unexpected first byte: {b}")
            }
            Self::ProxyError(e) => {
                write!(f, "ProxyError parse error: {e}")
            }
            Self::SecondByte(b) => {
                write!(f, "Unexpected second byte: {b}")
            }
            Self::ServerUnavailable(e) => {
                write!(f, "ServerUnavailable parse error: {e}")
            }
            Self::SlowDown(e) => {
                write!(f, "SlowDown parse error: {e}")
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
