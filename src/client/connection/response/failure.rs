pub mod error;
pub mod permanent;
pub mod temporary;

pub use error::Error;
pub use permanent::Permanent;
pub use temporary::Temporary;

pub enum Failure {
    /// 4* status code group
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#temporary-failure
    Temporary(Temporary),
    /// 5* status code group
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#permanent-failure
    Permanent(Permanent),
}

impl Failure {
    // Constructors

    /// Create new `Self` from buffer include header bytes
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        match buffer.first() {
            Some(b) => match b {
                b'4' => match Temporary::from_utf8(buffer) {
                    Ok(input) => Ok(Self::Temporary(input)),
                    Err(e) => Err(Error::Temporary(e)),
                },
                b'5' => match Permanent::from_utf8(buffer) {
                    Ok(failure) => Ok(Self::Permanent(failure)),
                    Err(e) => Err(Error::Permanent(e)),
                },
                b => Err(Error::Code(*b)),
            },
            None => Err(Error::Protocol),
        }
    }

    // Getters

    /// Get optional message for `Self`
    /// * return `None` if the message is empty
    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Permanent(permanent) => permanent.message(),
            Self::Temporary(temporary) => temporary.message(),
        }
    }

    /// Get optional message for `Self`
    /// * if the optional message not provided by the server, return children `DEFAULT_MESSAGE`
    pub fn message_or_default(&self) -> &str {
        match self {
            Self::Permanent(permanent) => permanent.message_or_default(),
            Self::Temporary(temporary) => temporary.message_or_default(),
        }
    }

    /// Get header string of `Self`
    pub fn as_str(&self) -> &str {
        match self {
            Self::Permanent(permanent) => permanent.as_str(),
            Self::Temporary(temporary) => temporary.as_str(),
        }
    }

    /// Get header bytes of `Self`
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Permanent(permanent) => permanent.as_bytes(),
            Self::Temporary(temporary) => temporary.as_bytes(),
        }
    }
}

#[test]
fn test() {
    fn t(source: String, message: Option<&str>) {
        let b = source.as_bytes();
        let i = Failure::from_utf8(b).unwrap();
        assert_eq!(i.message(), message);
        assert_eq!(i.as_str(), source);
        assert_eq!(i.as_bytes(), b);
    }
    for code in [40, 41, 42, 43, 44, 50, 51, 52, 53, 59] {
        t(format!("{code} Message\r\n"), Some("Message"));
        t(format!("{code}\r\n"), None);
    }
}
