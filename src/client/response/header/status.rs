pub mod error;
pub use error::Error;

use glib::GString;

/// https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes
#[derive(Debug)]
pub enum Status {
    Input,
    SensitiveInput,
    Success,
    Redirect,
} // @TODO

impl Status {
    pub fn from_header(buffer: &[u8]) -> Result<Self, Error> {
        match buffer.get(0..2) {
            Some(value) => match GString::from_utf8(value.to_vec()) {
                Ok(string) => Self::from_string(string.as_str()),
                Err(_) => Err(Error::Decode),
            },
            None => Err(Error::Undefined),
        }
    }

    pub fn from_string(code: &str) -> Result<Self, Error> {
        match code {
            "10" => Ok(Self::Input),
            "11" => Ok(Self::SensitiveInput),
            "20" => Ok(Self::Success),
            _ => Err(Error::Undefined),
        }
    }
}
