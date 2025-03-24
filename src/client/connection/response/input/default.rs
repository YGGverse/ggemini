pub mod error;
pub use error::Error;

/// [Input Expected](https://geminiprotocol.net/docs/protocol-specification.gmi#status-10) status code
pub const CODE: &[u8] = b"10";

/// Hold header `String` for [Input Expected](https://geminiprotocol.net/docs/protocol-specification.gmi#status-10) status code
/// * this response type does not contain body data
/// * the header member is closed to require valid construction
pub struct Default(String);

impl Default {
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
    let default = Default::from_utf8("10 Default\r\n".as_bytes()).unwrap();
    assert_eq!(default.message(), Some("Default"));
    assert_eq!(default.as_str(), "10 Default\r\n");

    let default = Default::from_utf8("10\r\n".as_bytes()).unwrap();
    assert_eq!(default.message(), None);
    assert_eq!(default.as_str(), "10\r\n");

    // err
    // @TODO assert!(Default::from_utf8("10Fail\r\n".as_bytes()).is_err());
    assert!(Default::from_utf8("13 Fail\r\n".as_bytes()).is_err());
    assert!(Default::from_utf8("Fail\r\n".as_bytes()).is_err());
    assert!(Default::from_utf8("Fail".as_bytes()).is_err());
}
