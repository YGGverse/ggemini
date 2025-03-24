pub mod error;
pub mod header;

pub use error::Error;
pub use header::Header;

pub const CODE: &[u8] = b"20";

pub struct Default {
    pub header: Header,
    pub content: Option<Vec<u8>>,
}

impl Default {
    // Constructors

    pub fn parse(buffer: &[u8]) -> Result<Self, Error> {
        if !buffer.starts_with(CODE) {
            return Err(Error::Code);
        }
        let header = Header::parse(buffer).map_err(Error::Header)?;
        Ok(Self {
            content: buffer
                .get(header.len() + 1..)
                .filter(|s| !s.is_empty())
                .map(|v| v.to_vec()),
            header,
        })
    }
}

#[test]
fn test() {
    let default = Default::parse("20 text/gemini; charset=utf-8; lang=en\r\n".as_bytes()).unwrap();
    assert_eq!(default.header.mime().unwrap(), "text/gemini");
    assert_eq!(default.content, None)
}
