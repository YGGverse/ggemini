use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Host,
    Scheme,
    Uri(glib::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Host => {
                write!(f, "Host required")
            }
            Self::Scheme => {
                write!(f, "Scope does not match `gemini`")
            }
            Self::Uri(reason) => {
                write!(f, "Could not parse URI: {reason}")
            }
        }
    }
}
