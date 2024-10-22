pub mod error;
pub mod meta;
pub mod mime;
pub mod status;

pub use error::Error;
pub use meta::Meta;
pub use mime::Mime;
pub use status::Status;

pub struct Header {
    status: Status,
    meta: Option<Meta>,
    mime: Option<Mime>,
    // @TODO
    // charset: Option<Charset>,
    // language: Option<Language>,
}

impl Header {
    /// Construct from response buffer
    /// https://geminiprotocol.net/docs/gemtext-specification.gmi#media-type-parameters
    pub fn from_response(response: &[u8] /* @TODO */) -> Result<Self, Error> {
        let end = Self::end(response)?;

        let buffer = match response.get(..end) {
            Some(result) => result,
            None => return Err(Error::Buffer),
        };

        let meta = match Meta::from_header(buffer) {
            Ok(result) => Some(result),
            Err(_) => None,
        };

        let mime = mime::from_header(buffer); // optional
                                              // let charset = charset::from_header(buffer); @TODO
                                              // let language = language::from_header(buffer); @TODO

        let status = match status::from_header(buffer) {
            Ok(result) => result,
            Err(_) => return Err(Error::Status),
        };

        Ok(Self { status, meta, mime })
    }

    // Getters
    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn mime(&self) -> &Option<Mime> {
        &self.mime
    }

    pub fn meta(&self) -> &Option<Meta> {
        &self.meta
    }

    // Tools
    fn end(buffer: &[u8]) -> Result<usize, Error> {
        for (offset, &byte) in buffer.iter().enumerate() {
            if byte == b'\r' {
                return Ok(offset);
            }
        }
        Err(Error::Format)
    }
}
