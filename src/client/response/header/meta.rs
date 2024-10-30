pub mod error;
pub use error::Error;

use glib::GString;

/// Response meta holder
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
        let mut bytes: Vec<u8> = Vec::new();

        // Skip status code
        match buffer.get(3..) {
            Some(slice) => {
                for (count, &byte) in slice.iter().enumerate() {
                    // Validate length
                    if count > 0x400 {
                        // 1024
                        return Err(Error::Protocol);
                    }

                    // End of line, done
                    if byte == b'\r' {
                        break;
                    }

                    // Append
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
