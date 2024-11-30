use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Data(super::data::Error),
    InputStream(Vec<u8>, glib::Error),
    Mime(super::mime::Error),
    Protocol,
    Status(super::status::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Data(e) => {
                write!(f, "Data error: {e}")
            }
            Self::InputStream(_, e) => {
                // @TODO
                write!(f, "Input stream error: {e}")
            }
            Self::Mime(e) => {
                write!(f, "MIME error: {e}")
            }
            Self::Protocol => {
                write!(f, "Protocol error")
            }
            Self::Status(e) => {
                write!(f, "Status error: {e}")
            }
        }
    }
}
