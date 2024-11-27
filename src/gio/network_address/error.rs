use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Host(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Host(url) => {
                write!(f, "Host required for {url}")
            }
        }
    }
}
