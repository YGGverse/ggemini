pub mod error;
pub use error::Error;

const CODE: &[u8] = b"61";

/// Hold header `String` for [61](https://geminiprotocol.net/docs/protocol-specification.gmi#status-61) status code
/// * this response type does not contain body data
/// * the header member is closed to require valid construction
pub struct NotAuthorized(String);

impl NotAuthorized {
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
    let not_authorized = NotAuthorized::from_utf8("61 Not Authorized\r\n".as_bytes()).unwrap();
    assert_eq!(not_authorized.message(), Some("Not Authorized"));
    assert_eq!(not_authorized.as_str(), "61 Not Authorized\r\n");

    let not_authorized = NotAuthorized::from_utf8("61\r\n".as_bytes()).unwrap();
    assert_eq!(not_authorized.message(), None);
    assert_eq!(not_authorized.as_str(), "61\r\n");

    // err
    assert!(NotAuthorized::from_utf8("62 Fail\r\n".as_bytes()).is_err());
    assert!(NotAuthorized::from_utf8("62 Fail\r\n".as_bytes()).is_err());
    assert!(NotAuthorized::from_utf8("Fail\r\n".as_bytes()).is_err());
    assert!(NotAuthorized::from_utf8("Fail".as_bytes()).is_err());
}
