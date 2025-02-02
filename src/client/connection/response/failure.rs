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
            Some(byte) => match byte {
                4 => match Temporary::from_utf8(buffer) {
                    Ok(input) => Ok(Self::Temporary(input)),
                    Err(e) => Err(Error::Temporary(e)),
                },
                5 => match Permanent::from_utf8(buffer) {
                    Ok(failure) => Ok(Self::Permanent(failure)),
                    Err(e) => Err(Error::Permanent(e)),
                },
                b => Err(Error::Code(*b)),
            },
            None => Err(Error::Protocol),
        }
    }

    // Getters

    pub fn to_code(&self) -> u8 {
        match self {
            Self::Permanent(permanent) => permanent.to_code(),
            Self::Temporary(temporary) => temporary.to_code(),
        }
    }

    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Permanent(permanent) => permanent.message(),
            Self::Temporary(temporary) => temporary.message(),
        }
    }
}
