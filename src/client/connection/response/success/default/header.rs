pub mod error;
pub use error::Error;

pub struct Header(String);

impl Header {
    // Constructors

    /// Parse `Self` from buffer contains header bytes
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        if !buffer.starts_with(super::CODE) {
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

    /// Parse content type for `Self`
    pub fn mime(&self) -> Result<String, Error> {
        glib::Regex::split_simple(
            r"^\d{2}\s([^\/]+\/[^\s;]+)",
            &self.0,
            glib::RegexCompileFlags::DEFAULT,
            glib::RegexMatchFlags::DEFAULT,
        )
        .get(1)
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map_or(Err(Error::Mime), |s| Ok(s.to_lowercase()))
    }

    /// Get header bytes of `Self`
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Get header string of `Self`
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[test]
fn test() {
    let s = "20 text/gemini; charset=utf-8; lang=en\r\n";
    let b = s.as_bytes();
    let h = Header::from_utf8(b).unwrap();
    assert_eq!(h.mime().unwrap(), "text/gemini");
    assert_eq!(h.as_bytes(), b);
    assert_eq!(h.as_str(), s);

    assert!(Header::from_utf8("21 text/gemini; charset=utf-8; lang=en\r\n".as_bytes()).is_err());
}
