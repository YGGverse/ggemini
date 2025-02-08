use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    BytesTotal(gio::MemoryInputStream, usize, usize),
    InputStream(gio::MemoryInputStream, glib::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::BytesTotal(_, total, limit) => {
                write!(f, "Bytes total limit reached: {total} / {limit}")
            }
            Self::InputStream(_, e) => {
                write!(f, "Input stream error: {e}")
            }
        }
    }
}
