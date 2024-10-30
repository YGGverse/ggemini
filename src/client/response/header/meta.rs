pub mod error;
pub use error::Error;

use glib::GString;

/// Response meta holder
///
/// Could be created from entire response buffer or just header slice
///
/// Use as:
/// * placeholder for 10, 11 status
/// * URL for 30, 31 status
pub struct Meta {
    value: Option<GString>,
}

impl Meta {
    /// Parse Meta from UTF-8
    pub fn from(buffer: &[u8]) -> Result<Self, Error> {
        // Init bytes buffer
        let mut bytes: Vec<u8> = Vec::with_capacity(1021);

        // Skip 3 bytes for status code of 1024 expected
        match buffer.get(3..1021) {
            Some(slice) => {
                for &byte in slice {
                    // End of header
                    if byte == b'\r' {
                        break;
                    }

                    // Continue
                    bytes.push(byte);
                }

                // Assumes the bytes are valid UTF-8
                match GString::from_utf8(bytes) {
                    Ok(value) => Ok(Self {
                        value: match value.is_empty() {
                            true => None,
                            false => Some(value),
                        },
                    }),
                    Err(_) => Err(Error::Decode),
                }
            }
            None => Err(Error::Protocol),
        }
    }

    pub fn value(&self) -> &Option<GString> {
        &self.value
    }
}
