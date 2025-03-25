pub mod default;
pub mod error;
pub mod sensitive;

pub use default::Default;
pub use error::Error;
pub use sensitive::Sensitive;

const CODE: u8 = b'1';

/// [Input expected](https://geminiprotocol.net/docs/protocol-specification.gmi#input-expected)
pub enum Input {
    Default(Default),
    Sensitive(Sensitive),
}

impl Input {
    // Constructors

    /// Create new `Self` from buffer include header bytes
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        match buffer.first() {
            Some(b) => match *b {
                CODE => match buffer.get(1) {
                    Some(b) => match *b {
                        b'0' => Ok(Self::Default(
                            Default::from_utf8(buffer).map_err(Error::Default)?,
                        )),
                        b'1' => Ok(Self::Sensitive(
                            Sensitive::from_utf8(buffer).map_err(Error::Sensitive)?,
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
            Self::Default(default) => default.message(),
            Self::Sensitive(sensitive) => sensitive.message(),
        }
    }

    /// Get optional message for `Self`
    /// * if the optional message not provided by the server, return children `DEFAULT_MESSAGE`
    pub fn message_or_default(&self) -> &str {
        match self {
            Self::Default(default) => default.message_or_default(),
            Self::Sensitive(sensitive) => sensitive.message_or_default(),
        }
    }

    /// Get header string of `Self`
    pub fn as_str(&self) -> &str {
        match self {
            Self::Default(default) => default.as_str(),
            Self::Sensitive(sensitive) => sensitive.as_str(),
        }
    }

    /// Get header bytes of `Self`
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Default(default) => default.as_bytes(),
            Self::Sensitive(sensitive) => sensitive.as_bytes(),
        }
    }
}

#[test]
fn test() {
    fn t(source: &str, message: Option<&str>) {
        let b = source.as_bytes();
        let i = Input::from_utf8(b).unwrap();
        assert_eq!(i.message(), message);
        assert_eq!(i.as_str(), source);
        assert_eq!(i.as_bytes(), b);
    }
    // 10
    t("10 Default\r\n", Some("Default"));
    t("10\r\n", None);
    // 11
    t("11 Sensitive\r\n", Some("Sensitive"));
    t("11\r\n", None);
}
