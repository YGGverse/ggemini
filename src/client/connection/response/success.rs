pub mod default;
pub mod error;

pub use default::Default;
pub use error::Error;

const CODE: u8 = b'2';

pub enum Success {
    Default(Default),
    // reserved for 2* codes
}

impl Success {
    // Constructors

    /// Parse new `Self` from buffer bytes
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        if buffer.first().is_none_or(|b| *b != CODE) {
            return Err(Error::Code);
        }
        match Default::from_utf8(buffer) {
            Ok(default) => Ok(Self::Default(default)),
            Err(e) => Err(Error::Default(e)),
        }
    }

    // Getters

    /// Get header bytes for `Self` type
    pub fn as_header_bytes(&self) -> &[u8] {
        match self {
            Self::Default(default) => default.header.as_bytes(),
        }
    }

    /// Get header string for `Self` type
    pub fn as_header_str(&self) -> &str {
        match self {
            Self::Default(default) => default.header.as_str(),
        }
    }

    /// Get parsed MIME for `Self` type
    ///
    /// * high-level method, useful to skip extra match case constructions;
    /// * at this moment, Gemini protocol has only one status code in this scope,\
    ///   this method would be deprecated in future, use on your own risk!
    pub fn mime(&self) -> Result<String, Error> {
        match self {
            Self::Default(default) => default
                .header
                .mime()
                .map_err(|e| Error::Default(default::Error::Header(e))),
        }
    }
}

#[test]
fn test() {
    let r = "20 text/gemini; charset=utf-8; lang=en\r\n";
    let b = r.as_bytes();
    let s = Success::from_utf8(b).unwrap();

    match s {
        Success::Default(ref d) => {
            assert_eq!(d.header.mime().unwrap(), "text/gemini");
            assert!(d.content.is_empty())
        }
    }
    assert_eq!(s.as_header_bytes(), b);
    assert_eq!(s.as_header_str(), r);
    assert_eq!(s.mime().unwrap(), "text/gemini");

    assert!(Success::from_utf8("40 text/gemini; charset=utf-8; lang=en\r\n".as_bytes()).is_err())
}
