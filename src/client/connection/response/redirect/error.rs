use std::{
    fmt::{Display, Formatter, Result},
    str::Utf8Error,
};

#[derive(Debug)]
pub enum Error {
    Glib(glib::Error),
    Protocol,
    Target,
    Utf8Error(Utf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Glib(e) => {
                write!(f, "Glib error: {e}")
            }
            Self::Utf8Error(e) => {
                write!(f, "UTF-8 error: {e}")
            }
            Self::Protocol => {
                write!(f, "Protocol error")
            }
            Self::Target => {
                write!(f, "Target error")
            }
        }
    }
}
