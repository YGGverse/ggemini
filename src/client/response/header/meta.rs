pub mod error;
pub use error::Error;

use glib::GString;

/// Entire meta buffer, but [status code](https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes).
///
/// Useful to grab placeholder text on 10, 11, 31 codes processing
pub struct Meta {
    buffer: Vec<u8>,
}

impl Meta {
    pub fn from_header(buffer: &[u8]) -> Result<Self, Error> {
        match buffer.get(3..) {
            Some(value) => Ok(Self {
                buffer: value.to_vec(),
            }),
            None => return Err(Error::Protocol),
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
