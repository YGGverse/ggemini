pub mod error;
pub use error::Error;

use glib::{Bytes, GString};

pub struct Body {
    buffer: Vec<u8>,
}

impl Body {
    // Constructors
    pub fn from_response(bytes: &Bytes) -> Result<Self, Error> {
        let start = Self::start(bytes)?;

        let buffer = match bytes.get(start..) {
            Some(result) => result,
            None => return Err(Error::Buffer),
        };

        Ok(Self {
            buffer: Vec::from(buffer),
        })
    }

    // Getters
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn to_gstring(&self) -> Result<GString, Error> {
        match GString::from_utf8(self.buffer.to_vec()) {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::Decode),
        }
    }

    // Tools
    fn start(buffer: &[u8]) -> Result<usize, Error> {
        for (offset, &byte) in buffer.iter().enumerate() {
            if byte == b'\n' {
                return Ok(offset + 1);
            }
        }
        Err(Error::Format)
    }
}
