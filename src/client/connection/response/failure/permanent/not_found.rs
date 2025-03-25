pub mod error;
pub use error::Error;

/// [Not Found Permanent Error](https://geminiprotocol.net/docs/protocol-specification.gmi#status-51-not-found) status code
pub const CODE: &[u8] = b"51";

/// Default message if the optional value was not provided by the server
/// * useful to skip match cases in external applications,
///   by using `super::message_or_default` method.
pub const DEFAULT_MESSAGE: &str = "Not Found";

/// Hold header `String` for [Not Found Permanent Error](https://geminiprotocol.net/docs/protocol-specification.gmi#status-51-not-found) status code
/// * this response type does not contain body data
/// * the header member is closed to require valid construction
pub struct NotFound(String);

impl NotFound {
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

    /// Get optional message for `Self`
    /// * if the optional message not provided by the server, return `DEFAULT_MESSAGE`
    pub fn message_or_default(&self) -> &str {
        self.message().unwrap_or(DEFAULT_MESSAGE)
    }

    /// Get header string of `Self`
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get header bytes of `Self`
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[test]
fn test() {
    // ok
    let nf = NotFound::from_utf8("51 Message\r\n".as_bytes()).unwrap();
    assert_eq!(nf.message(), Some("Message"));
    assert_eq!(nf.message_or_default(), "Message");
    assert_eq!(nf.as_str(), "51 Message\r\n");
    assert_eq!(nf.as_bytes(), "51 Message\r\n".as_bytes());

    let nf = NotFound::from_utf8("51\r\n".as_bytes()).unwrap();
    assert_eq!(nf.message(), None);
    assert_eq!(nf.message_or_default(), DEFAULT_MESSAGE);
    assert_eq!(nf.as_str(), "51\r\n");
    assert_eq!(nf.as_bytes(), "51\r\n".as_bytes());

    // err
    assert!(NotFound::from_utf8("13 Fail\r\n".as_bytes()).is_err());
    assert!(NotFound::from_utf8("Fail\r\n".as_bytes()).is_err());
    assert!(NotFound::from_utf8("Fail".as_bytes()).is_err());
}
