pub mod error;
pub use error::Error;

use glib::GString;

/// Response meta holder, but [status code](https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes).
///
/// Use as:
/// * placeholder for 10, 11 status
/// * URL for 30, 31 status
pub struct Meta {
    buffer: Vec<u8>,
}

impl Meta {
    pub fn from_header(buffer: &[u8]) -> Result<Self, Error> {
        match buffer.get(3..) {
            Some(value) => Ok(Self {
                buffer: value.to_vec(),
            }),
            None => return Err(Error::Protocol), // @TODO optional
        }
    }

    pub fn to_gstring(&self) -> Result<GString, Error> {
        match GString::from_utf8(self.buffer.clone()) {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::Decode),
        }
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}
