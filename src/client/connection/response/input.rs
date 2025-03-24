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

    pub fn as_str(&self) -> &str {
        match self {
            Self::Default(default) => default.as_str(),
            Self::Sensitive(sensitive) => sensitive.as_str(),
        }
    }

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
        let bytes = source.as_bytes();
        let input = Input::from_utf8(bytes).unwrap();
        assert_eq!(input.message(), message);
        assert_eq!(input.as_str(), source);
        assert_eq!(input.as_bytes(), bytes);
    }
    // 10
    t("10 Default\r\n", Some("Default"));
    t("10\r\n", None);
    // 11
    t("11 Sensitive\r\n", Some("Sensitive"));
    t("11\r\n", None);
}
