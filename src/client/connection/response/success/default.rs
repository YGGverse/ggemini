pub mod error;
pub mod header;

pub use error::Error;
pub use header::Header;

/// [Success](https://geminiprotocol.net/docs/protocol-specification.gmi#success) status code
pub const CODE: &[u8] = b"20";

/// Holder for [Success](https://geminiprotocol.net/docs/protocol-specification.gmi#success) status code
/// * this response type MAY contain body data
/// * the header has closed members to require valid construction
pub struct Default {
    pub header: Header,
    pub content: Option<Vec<u8>>,
}

impl Default {
    // Constructors

    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        if !buffer.starts_with(CODE) {
            return Err(Error::Code);
        }
        let header = Header::from_utf8(buffer).map_err(Error::Header)?;
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
    let default =
        Default::from_utf8("20 text/gemini; charset=utf-8; lang=en\r\n".as_bytes()).unwrap();
    assert_eq!(default.header.mime().unwrap(), "text/gemini");
    assert_eq!(default.content, None)
}
