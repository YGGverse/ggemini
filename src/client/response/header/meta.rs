pub mod error;
pub use error::Error;

use glib::GString;

pub struct Meta {
    buffer: Vec<u8>,
}

impl Meta {
    pub fn from_header(buffer: &[u8] /* @TODO */) -> Result<Self, Error> {
        let buffer = match buffer.get(2..) {
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
}
