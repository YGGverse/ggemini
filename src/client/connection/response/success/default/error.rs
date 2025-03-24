use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Code,
    Header(super::header::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Code => {
                write!(f, "Unexpected status code")
            }
            Self::Header(e) => {
                write!(f, "Header error: {e}")
            }
        }
    }
}
