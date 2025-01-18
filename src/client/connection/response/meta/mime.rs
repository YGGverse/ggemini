//! MIME type parser for different data types

pub mod error;
pub use error::Error;

use glib::{Regex, RegexCompileFlags, RegexMatchFlags};

/// MIME type holder for `Response` (by [Gemtext specification](https://geminiprotocol.net/docs/gemtext-specification.gmi#media-type-parameters))
/// * the value stored in lowercase
pub struct Mime(String);

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
            Some(utf8) => match std::str::from_utf8(utf8) {
                Ok(s) => Self::from_string(s),
                Err(e) => Err(Error::Decode(e)),
            },
            None => Err(Error::Protocol),
        }
    }

    /// Create new `Self` from `str::str` that includes **header**
    /// * return `None` for non 2* [status codes](https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes)
    pub fn from_string(s: &str) -> Result<Option<Self>, Error> {
        if !s.starts_with("2") {
            return Ok(None);
        }
        match parse(s) {
            Some(v) => Ok(Some(Self(v))),
            None => Err(Error::Undefined),
        }
    }

    // Getters

    /// Get `Self` as lowercase `std::str`
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for Mime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Extract MIME type from from string that includes **header**
pub fn parse(s: &str) -> Option<String> {
    Regex::split_simple(
        r"^2\d{1}\s([^\/]+\/[^\s;]+)",
        s,
        RegexCompileFlags::DEFAULT,
        RegexMatchFlags::DEFAULT,
    )
    .get(1)
    .map(|this| this.to_lowercase())
}
