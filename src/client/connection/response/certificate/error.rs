use std::{
    fmt::{Display, Formatter, Result},
    str::Utf8Error,
};

#[derive(Debug)]
pub enum Error {
    Code,
    Utf8Error(Utf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Code => {
                write!(f, "Status code error")
            }
            Self::Utf8Error(e) => {
                write!(f, "UTF-8 error: {e}")
            }
        }
    }
}
