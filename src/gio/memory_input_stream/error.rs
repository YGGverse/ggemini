use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    BytesTotal(usize, usize),
    InputStream(glib::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::BytesTotal(total, limit) => {
                write!(f, "Bytes total limit reached: {total} / {limit}")
            }
            Self::InputStream(e) => {
                write!(f, "Input stream error: {e}")
            }
        }
    }
}
