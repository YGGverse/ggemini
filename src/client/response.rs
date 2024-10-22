pub mod body;
pub mod error;
pub mod header;

pub use body::Body;
pub use error::Error;
pub use header::Header;

pub struct Response {
    header: Header,
    body: Body,
}

impl Response {
    /// Create new `client::Response`
    pub fn new(header: Header, body: Body) -> Self {
        Self { header, body }
    }

    /// Create new `client::Response` from UTF-8 buffer
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        let header = match Header::from_response(buffer) {
            Ok(result) => result,
            Err(_) => return Err(Error::Header),
        };

        let body = match Body::from_response(buffer) {
            Ok(result) => result,
            Err(_) => return Err(Error::Body),
        };

        Ok(Self::new(header, body))
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn body(&self) -> &Body {
        &self.body
    }
}
