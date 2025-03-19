use std::{
    fmt::{Display, Formatter, Result},
    str::Utf8Error,
};

#[derive(Debug)]
pub enum Error {
    Code,
    HeaderLen(usize),
    Utf8Error(Utf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Code => {
                write!(f, "Unexpected status code")
            }
            Self::HeaderLen(l) => {
                write!(
                    f,
                    "Header length reached protocol limit ({l} of {} bytes max)",
                    super::super::HEADER_LEN
                )
            }
            Self::Utf8Error(e) => {
                write!(f, "UTF-8 error: {e}")
            }
        }
    }
}
