use std::{
    fmt::{Display, Formatter, Result},
    str::Utf8Error,
};

#[derive(Debug)]
pub enum Error {
    Certificate(super::certificate::Error),
    Code(u8),
    Failure(super::failure::Error),
    Input(super::input::Error),
    Protocol,
    Redirect(super::redirect::Error),
    Stream(glib::Error, Vec<u8>),
    Success(super::success::Error),
    Utf8Error(Utf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Certificate(e) => {
                write!(f, "Certificate error: {e}")
            }
            Self::Code(e) => {
                write!(f, "Code group error: {e}*")
            }
            Self::Failure(e) => {
                write!(f, "Failure error: {e}")
            }
            Self::Input(e) => {
                write!(f, "Input error: {e}")
            }
            Self::Protocol => {
                write!(f, "Protocol error")
            }
            Self::Redirect(e) => {
                write!(f, "Redirect error: {e}")
            }
            Self::Stream(e, ..) => {
                write!(f, "I/O stream error: {e}")
            }
            Self::Success(e) => {
                write!(f, "Success error: {e}")
            }
            Self::Utf8Error(e) => {
                write!(f, "UTF-8 decode error: {e}")
            }
        }
    }
}
