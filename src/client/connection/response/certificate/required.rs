pub mod error;
pub use error::Error;

/// [Certificate Required](https://geminiprotocol.net/docs/protocol-specification.gmi#status-60) status code
pub const CODE: &[u8] = b"60";

/// Hold header `String` for [Certificate Required](https://geminiprotocol.net/docs/protocol-specification.gmi#status-60) status code
/// * this response type does not contain body data
/// * the header member is closed to require valid construction
pub struct Required(String);

impl Required {
    // Constructors

    /// Parse `Self` from buffer contains header bytes
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        if !buffer.starts_with(CODE) {
            return Err(Error::Code);
        }
        Ok(Self(
            std::str::from_utf8(
                crate::client::connection::response::header_bytes(buffer).map_err(Error::Header)?,
            )
            .map_err(Error::Utf8Error)?
            .to_string(),
        ))
    }

    // Getters

    /// Get optional message for `Self`
    /// * return `None` if the message is empty
    pub fn message(&self) -> Option<&str> {
        self.0.get(2..).map(|s| s.trim()).filter(|x| !x.is_empty())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[test]
fn test() {
    // ok
    let required = Required::from_utf8("60 Required\r\n".as_bytes()).unwrap();
    assert_eq!(required.message(), Some("Required"));
    assert_eq!(required.as_str(), "60 Required\r\n");

    let required = Required::from_utf8("60\r\n".as_bytes()).unwrap();
    assert_eq!(required.message(), None);
    assert_eq!(required.as_str(), "60\r\n");

    // err
    assert!(Required::from_utf8("62 Fail\r\n".as_bytes()).is_err());
    assert!(Required::from_utf8("62 Fail\r\n".as_bytes()).is_err());
    assert!(Required::from_utf8("Fail\r\n".as_bytes()).is_err());
    assert!(Required::from_utf8("Fail".as_bytes()).is_err());
}
