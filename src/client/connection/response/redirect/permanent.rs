pub mod error;
pub use error::Error;

// Local dependencies

use glib::Uri;

/// [Permanent Redirection](https://geminiprotocol.net/docs/protocol-specification.gmi#status-31-permanent-redirection) status code
pub const CODE: &[u8] = b"31";

/// Hold header `String` for [Permanent Redirection](https://geminiprotocol.net/docs/protocol-specification.gmi#status-31-permanent-redirection) status code
/// * this response type does not contain body data
/// * the header member is closed to require valid construction
pub struct Permanent(String);

impl Permanent {
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

    /// Get raw target for `Self`
    /// * return `Err` if the required target is empty
    pub fn target(&self) -> Result<&str, Error> {
        self.0
            .get(2..)
            .map(|s| s.trim())
            .filter(|x| !x.is_empty())
            .ok_or(Error::TargetEmpty)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn uri(&self, base: &Uri) -> Result<Uri, Error> {
        super::uri(self.target()?, base).map_err(Error::Uri)
    }
}

#[test]
fn test() {
    const BUFFER: &str = "31 gemini://geminiprotocol.net/path\r\n";
    let base = Uri::build(
        glib::UriFlags::NONE,
        "gemini",
        None,
        Some("geminiprotocol.net"),
        -1,
        "/path/",
        Some("query"),
        Some("fragment"),
    );
    let permanent = Permanent::from_utf8(BUFFER.as_bytes()).unwrap();
    assert!(permanent.target().is_ok());
    assert!(
        permanent
            .uri(&base)
            .is_ok_and(|u| u.to_string() == "gemini://geminiprotocol.net/path")
    );
    assert!(Permanent::from_utf8("32 gemini://geminiprotocol.net/path\r\n".as_bytes()).is_err());
}
