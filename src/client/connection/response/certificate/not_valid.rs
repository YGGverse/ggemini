pub mod error;
pub use error::Error;

const CODE: &[u8] = b"62";

/// Hold header `String` for [62](https://geminiprotocol.net/docs/protocol-specification.gmi#status-62) status code
/// * this response type does not contain body data
/// * the header member is closed to require valid construction
pub struct NotValid(String);

impl NotValid {
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
    let not_valid = NotValid::from_utf8("62 Not Valid\r\n".as_bytes()).unwrap();
    assert_eq!(not_valid.message(), Some("Not Valid"));
    assert_eq!(not_valid.as_str(), "62 Not Valid\r\n");

    let not_valid = NotValid::from_utf8("62\r\n".as_bytes()).unwrap();
    assert_eq!(not_valid.message(), None);
    assert_eq!(not_valid.as_str(), "62\r\n");

    // err
    // @TODO assert!(NotValid::from_utf8("62Fail\r\n".as_bytes()).is_err());
    assert!(NotValid::from_utf8("63 Fail\r\n".as_bytes()).is_err());
    assert!(NotValid::from_utf8("Fail\r\n".as_bytes()).is_err());
    assert!(NotValid::from_utf8("Fail".as_bytes()).is_err());
}
