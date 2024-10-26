pub mod error;
pub use error::Error;

use glib::{Bytes, GString};

/// Entire meta buffer, but [status code](https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes).
///
/// Usefult to grab placeholder text on 10, 11, 31 codes processing
pub struct Meta {
    buffer: Vec<u8>,
}

impl Meta {
    pub fn from_header(bytes: &Bytes) -> Result<Self, Error> {
        let buffer = match bytes.get(3..) {
            Some(bytes) => bytes.to_vec(),
            None => return Err(Error::Undefined),
        };

        Ok(Self { buffer })
    }

    pub fn to_gstring(&self) -> Result<GString, Error> {
        match GString::from_utf8(self.buffer.clone()) {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::Undefined),
        }
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}
