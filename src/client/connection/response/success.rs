pub mod default;
pub mod error;

pub use default::Default;
pub use error::Error;

pub const CODE: u8 = b'2';

pub enum Success {
    Default(Default),
    // reserved for 2* codes
}

impl Success {
    // Constructors

    /// Parse new `Self` from buffer bytes
    pub fn parse(buffer: &[u8]) -> Result<Self, Error> {
        if !buffer.first().is_some_and(|b| *b == CODE) {
            return Err(Error::Code);
        }
        match Default::parse(&buffer) {
            Ok(default) => Ok(Self::Default(default)),
            Err(e) => Err(Error::Default(e)),
        }
    }
}

#[test]
fn test() {
    // let default = Success::parse("20 text/gemini; charset=utf-8; lang=en\r\n".as_bytes());
    todo!()
}
