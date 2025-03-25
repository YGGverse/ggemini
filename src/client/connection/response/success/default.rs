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
    /// Formatted header holder with additional API
    pub header: Header,
    /// Default success response MAY include body data
    /// * if the `Request` constructed with `Mode::HeaderOnly` flag,\
    ///   this value wants to be processed manually, using external application logic (specific for content-type)
    pub content: Vec<u8>,
}

impl Default {
    // Constructors

    /// Parse `Self` from buffer contains header bytes
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        if !buffer.starts_with(CODE) {
            return Err(Error::Code);
        }
        let header = Header::from_utf8(buffer).map_err(Error::Header)?;
        Ok(Self {
            content: buffer
                .get(header.as_bytes().len()..)
                .filter(|s| !s.is_empty())
                .map_or(Vec::new(), |v| v.to_vec()),
            header,
        })
    }
}

#[test]
fn test() {
    let d = Default::from_utf8("20 text/gemini; charset=utf-8; lang=en\r\n".as_bytes()).unwrap();
    assert_eq!(d.header.mime().unwrap(), "text/gemini");
    assert!(d.content.is_empty());

    let d =
        Default::from_utf8("20 text/gemini; charset=utf-8; lang=en\r\ndata".as_bytes()).unwrap();
    assert_eq!(d.header.mime().unwrap(), "text/gemini");
    assert_eq!(d.content.len(), 4);
}
