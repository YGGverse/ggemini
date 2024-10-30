pub mod error;
pub use error::Error;

use glib::GString;

/// https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes
#[derive(Debug)]
pub enum Status {
    // 10 | 11
    Input,
    SensitiveInput,
    // 20
    Success,
    // 30 | 31
    Redirect,
    PermanentRedirect,
} // @TODO

impl Status {
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        match buffer.get(0..2) {
            Some(value) => match GString::from_utf8(value.to_vec()) {
                Ok(string) => Self::from_string(string.as_str()),
                Err(_) => Err(Error::Decode),
            },
            None => Err(Error::Protocol),
        }
    }

    pub fn from_string(code: &str) -> Result<Self, Error> {
        match code {
            "10" => Ok(Self::Input),
            "11" => Ok(Self::SensitiveInput),
            "20" => Ok(Self::Success),
            "30" => Ok(Self::Redirect),
            "31" => Ok(Self::PermanentRedirect),
            _ => Err(Error::Undefined),
        }
    }
}
