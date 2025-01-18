//! Components for reading and parsing meta **data** bytes from response
//! (e.g. placeholder text for 10, 11, url string for 30, 31 etc)

pub mod error;
pub use error::Error;

/// Meta **data** holder
///
/// For example, `value` could contain:
/// * placeholder text for 10, 11 status
/// * URL string for 30, 31 status
pub struct Data(String);

impl Data {
    // Constructors

    /// Parse meta **data** from UTF-8 buffer
    /// from entire response or just header slice
    ///
    /// * result could be `None` for some [status codes](https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes)
    ///   that does not expect any data in header
    pub fn from_utf8(buffer: &[u8]) -> Result<Option<Self>, Error> {
        // Define max buffer length for this method
        const MAX_LEN: usize = 0x400; // 1024

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
                match String::from_utf8(bytes) {
                    Ok(data) => Ok(match data.is_empty() {
                        false => Some(Self(data)),
                        true => None,
                    }),
                    Err(e) => Err(Error::Decode(e)),
                }
            }
            None => Err(Error::Protocol),
        }
    }

    // Getters

    /// Get `Self` as `std::str`
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
