use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    NetworkAddress(crate::gio::network_address::error::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::NetworkAddress(e) => {
                write!(f, "Network Address error: {e}")
            }
        }
    }
}
