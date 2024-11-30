use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Meta(super::meta::Error),
    Stream,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Meta(e) => {
                write!(f, "Meta read error: {e}")
            }
            Self::Stream => {
                write!(f, "I/O stream error")
            }
        }
    }
}
