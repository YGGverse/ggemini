use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Code,
    Permanent(super::permanent::Error),
    Protocol,
    Temporary(super::temporary::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Code => {
                write!(f, "Code group error")
            }
            Self::Permanent(e) => {
                write!(f, "Permanent failure group error: {e}")
            }
            Self::Protocol => {
                write!(f, "Protocol error")
            }
            Self::Temporary(e) => {
                write!(f, "Temporary failure group error: {e}")
            }
        }
    }
}
