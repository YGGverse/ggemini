pub mod error;
pub mod not_authorized;
pub mod not_valid;
pub mod required;

pub use error::Error;
pub use not_authorized::NotAuthorized;
pub use not_valid::NotValid;
pub use required::Required;

const CODE: u8 = b'6';

/// 6* status code group
/// https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates
pub enum Certificate {
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-60
    Required(Required),
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-61-certificate-not-authorized
    NotAuthorized(NotAuthorized),
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-62-certificate-not-valid
    NotValid(NotValid),
}

impl Certificate {
    // Constructors

    /// Create new `Self` from buffer include header bytes
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        match buffer.first() {
            Some(b) => match *b {
                CODE => match buffer.get(1) {
                    Some(b) => match *b {
                        b'0' => Ok(Self::Required(
                            Required::from_utf8(buffer).map_err(Error::Required)?,
                        )),
                        b'1' => Ok(Self::NotAuthorized(
                            NotAuthorized::from_utf8(buffer).map_err(Error::NotAuthorized)?,
                        )),
                        b'2' => Ok(Self::NotValid(
                            NotValid::from_utf8(buffer).map_err(Error::NotValid)?,
                        )),
                        b => Err(Error::SecondByte(b)),
                    },
                    None => Err(Error::UndefinedSecondByte),
                },
                b => Err(Error::FirstByte(b)),
            },
            None => Err(Error::UndefinedFirstByte),
        }
    }

    // Getters

    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Required(required) => required.message(),
            Self::NotAuthorized(not_authorized) => not_authorized.message(),
            Self::NotValid(not_valid) => not_valid.message(),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Required(required) => required.as_str(),
            Self::NotAuthorized(not_authorized) => not_authorized.as_str(),
            Self::NotValid(not_valid) => not_valid.as_str(),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Required(required) => required.as_bytes(),
            Self::NotAuthorized(not_authorized) => not_authorized.as_bytes(),
            Self::NotValid(not_valid) => not_valid.as_bytes(),
        }
    }
}

#[test]
fn test() {
    fn t(source: &str, message: Option<&str>) {
        let b = source.as_bytes();
        let c = Certificate::from_utf8(b).unwrap();
        assert_eq!(c.message(), message);
        assert_eq!(c.as_str(), source);
        assert_eq!(c.as_bytes(), b);
    }
    // 60
    t("60 Required\r\n", Some("Required"));
    t("60\r\n", None);
    // 61
    t("61 Not Authorized\r\n", Some("Not Authorized"));
    t("61\r\n", None);
    // 62
    t("61 Not Valid\r\n", Some("Not Valid"));
    t("61\r\n", None);
}
