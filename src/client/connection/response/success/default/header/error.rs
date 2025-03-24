use std::{
    fmt::{Display, Formatter, Result},
    str::Utf8Error,
};

#[derive(Debug)]
pub enum Error {
    Code,
    Mime,
    Header(crate::client::connection::response::HeaderBytesError),
    Utf8Error(Utf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Code => {
                write!(f, "Unexpected status code")
            }
            Self::Mime => {
                write!(f, "Unexpected content type")
            }
            Self::Header(e) => {
                write!(f, "Header error: {e}")
            }
            Self::Utf8Error(e) => {
                write!(f, "UTF-8 error: {e}")
            }
        }
    }
}
