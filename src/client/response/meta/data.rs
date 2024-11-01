pub mod error;
pub use error::Error;

use glib::GString;

pub const MAX_LEN: usize = 0x400; // 1024

/// Meta data holder for response
///
/// Could be created from entire response buffer or just header slice
///
/// Use as:
/// * placeholder for 10, 11 status
/// * URL for 30, 31 status
pub struct Data {
    value: GString,
}

impl Data {
    /// Parse meta data from UTF-8 buffer
    ///
    /// * result could be `None` for some [status codes](https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes)
    /// that does not expect any data in header
    pub fn from_utf8(buffer: &[u8]) -> Result<Option<Self>, Error> {
        // Init bytes buffer
        let mut bytes: Vec<u8> = Vec::with_capacity(MAX_LEN);

        // Calculate len once
        let len = buffer.len();

        // Skip 3 bytes for status code of `MAX_LEN` expected
        match buffer.get(3..if len > MAX_LEN { MAX_LEN - 3 } else { len }) {
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
                    Ok(value) => Ok(match value.is_empty() {
                        false => Some(Self { value }),
                        true => None,
                    }),
                    Err(_) => Err(Error::Decode),
                }
            }
            None => Err(Error::Protocol),
        }
    }

    pub fn value(&self) -> &GString {
        &self.value
    }
}
