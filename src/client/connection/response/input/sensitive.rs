pub mod error;
pub use error::Error;

const CODE: &[u8] = b"11";

/// Hold header `String` for [11](https://geminiprotocol.net/docs/protocol-specification.gmi#status-11-sensitive-input) status code
/// * this response type does not contain body data
/// * the header member is closed to require valid construction
pub struct Sensitive(String);

impl Sensitive {
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
    let sensitive = Sensitive::from_utf8("11 Sensitive\r\n".as_bytes()).unwrap();
    assert_eq!(sensitive.message(), Some("Sensitive"));
    assert_eq!(sensitive.as_str(), "11 Sensitive\r\n");

    let sensitive = Sensitive::from_utf8("11\r\n".as_bytes()).unwrap();
    assert_eq!(sensitive.message(), None);
    assert_eq!(sensitive.as_str(), "11\r\n");

    // err
    // @TODO assert!(Sensitive::from_utf8("11Fail\r\n".as_bytes()).is_err());
    assert!(Sensitive::from_utf8("13 Fail\r\n".as_bytes()).is_err());
    assert!(Sensitive::from_utf8("Fail\r\n".as_bytes()).is_err());
    assert!(Sensitive::from_utf8("Fail".as_bytes()).is_err());
}
