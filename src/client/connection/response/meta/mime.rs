//! MIME type parser for different data types

pub mod error;
pub use error::Error;

use glib::{GString, Regex, RegexCompileFlags, RegexMatchFlags};

/// MIME type holder for `Response` (by [Gemtext specification](https://geminiprotocol.net/docs/gemtext-specification.gmi#media-type-parameters))
/// * the value stored in lowercase
pub struct Mime(String);

impl std::fmt::Display for Mime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Mime {
    // Constructors

    /// Create new `Self` from UTF-8 buffer (that includes **header**)
    /// * return `None` for non 2* [status codes](https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes)
    pub fn from_utf8(buffer: &[u8]) -> Result<Option<Self>, Error> {
        // Define max buffer length for this method
        const MAX_LEN: usize = 0x400; // 1024

        // Calculate buffer length once
        let len = buffer.len();

        // Parse meta bytes only
        match buffer.get(..if len > MAX_LEN { MAX_LEN } else { len }) {
            Some(value) => match GString::from_utf8(value.into()) {
                Ok(string) => Self::from_string(&string),
                Err(e) => Err(Error::Decode(e)),
            },
            None => Err(Error::Protocol),
        }
    }

    /// Create new `Self` from string that includes **header**
    /// * return `None` for non 2* [status codes](https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes)
    pub fn from_string(subject: &str) -> Result<Option<Self>, Error> {
        if !subject.starts_with("2") {
            return Ok(None);
        }
        match parse(subject) {
            Some(value) => Ok(Some(Self(value.to_lowercase()))),
            None => Err(Error::Undefined),
        }
    }

    // Getters

    /// Get `Self` as lowercase `std::str`
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Extract MIME type from from string that includes **header**
pub fn parse(value: &str) -> Option<String> {
    Regex::split_simple(
        r"^2\d{1}\s([^\/]+\/[^\s;]+)",
        value,
        RegexCompileFlags::DEFAULT,
        RegexMatchFlags::DEFAULT,
    )
    .get(1)
    .map(|this| this.to_string())
}
