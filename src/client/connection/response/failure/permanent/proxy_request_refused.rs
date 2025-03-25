pub mod error;
pub use error::Error;

/// [Proxy Request Refused](https://geminiprotocol.net/docs/protocol-specification.gmi#status-53-proxy-request-refused) permanent error status code
pub const CODE: &[u8] = b"53";

/// Default message if the optional value was not provided by the server
/// * useful to skip match cases in external applications,
///   by using `super::message_or_default` method.
pub const DEFAULT_MESSAGE: &str = "Proxy request refused";

/// Hold header `String` for [Proxy Request Refused](https://geminiprotocol.net/docs/protocol-specification.gmi#status-53-proxy-request-refused) permanent error status code
/// * this response type does not contain body data
/// * the header member is closed to require valid construction
pub struct ProxyRequestRefused(String);

impl ProxyRequestRefused {
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
    let prf = ProxyRequestRefused::from_utf8("53 Message\r\n".as_bytes()).unwrap();
    assert_eq!(prf.message(), Some("Message"));
    assert_eq!(prf.message_or_default(), "Message");
    assert_eq!(prf.as_str(), "53 Message\r\n");
    assert_eq!(prf.as_bytes(), "53 Message\r\n".as_bytes());

    let prf = ProxyRequestRefused::from_utf8("53\r\n".as_bytes()).unwrap();
    assert_eq!(prf.message(), None);
    assert_eq!(prf.message_or_default(), DEFAULT_MESSAGE);
    assert_eq!(prf.as_str(), "53\r\n");
    assert_eq!(prf.as_bytes(), "53\r\n".as_bytes());

    // err
    assert!(ProxyRequestRefused::from_utf8("13 Fail\r\n".as_bytes()).is_err());
    assert!(ProxyRequestRefused::from_utf8("Fail\r\n".as_bytes()).is_err());
    assert!(ProxyRequestRefused::from_utf8("Fail".as_bytes()).is_err());
}
