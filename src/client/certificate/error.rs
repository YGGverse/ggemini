use std::fmt::{Display, Formatter, Result};

use glib::gformat;

#[derive(Debug)]
pub enum Error {
    DateTime,
    Decode(glib::Error),
    Expired(glib::DateTime),
    Inactive(glib::DateTime),
    Scope(crate::client::certificate::scope::Error),
    ValidAfter,
    ValidBefore,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::DateTime => {
                write!(f, "Could not parse local `DateTime`")
            }
            Self::Decode(reason) => {
                write!(
                    f,
                    "Could not decode TLS certificate from PEM string: {reason}"
                )
            }
            Self::Expired(not_valid_after) => {
                write!(
                    f,
                    "Certificate expired after: {}",
                    match not_valid_after.format_iso8601() {
                        Ok(value) => value,
                        Err(_) => gformat!("unknown"),
                    }
                )
            }
            Self::Inactive(not_valid_before) => {
                write!(
                    f,
                    "Certificate inactive before: {}",
                    match not_valid_before.format_iso8601() {
                        Ok(value) => value,
                        Err(_) => gformat!("unknown"),
                    }
                )
            }
            Self::Scope(reason) => {
                write!(f, "Certificate inactive before: {reason}")
            }
            Self::ValidAfter => write!(f, "Could not get `not_valid_after` value"),
            Self::ValidBefore => write!(f, "Could not get `not_valid_before` value"),
        }
    }
}
