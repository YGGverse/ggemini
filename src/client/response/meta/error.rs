use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Data(super::data::Error),
    InputStreamRead(Vec<u8>, glib::Error),
    Mime(super::mime::Error),
    Protocol,
    Status(super::status::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Data(reason) => {
                write!(f, "Data error: {reason}")
            }
            Self::InputStreamRead(_, reason) => {
                // @TODO
                write!(f, "Input stream error: {reason}")
            }
            Self::Mime(reason) => {
                write!(f, "MIME error: {reason}")
            }
            Self::Protocol => {
                write!(f, "Protocol error")
            }
            Self::Status(reason) => {
                write!(f, "Status error: {reason}")
            }
        }
    }
}
