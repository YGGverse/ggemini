pub mod error;
pub mod meta;
pub mod mime;
pub mod status;

pub use error::Error;
pub use meta::Meta;
pub use mime::Mime;
pub use status::Status;

use glib::Bytes;

pub struct Header {
    status: Status,
    meta: Option<Meta>,
    mime: Option<Mime>,
    // @TODO
    // charset: Option<Charset>,
    // language: Option<Language>,
}

impl Header {
    // Constructors
    pub fn from_response(bytes: &Bytes) -> Result<Self, Error> {
        // Get header slice of bytes
        let end = Self::end(bytes)?;

        let bytes = Bytes::from(match bytes.get(..end) {
            Some(buffer) => buffer,
            None => return Err(Error::Buffer),
        });

        // Status is required, parse to continue
        let status = match Status::from_header(&bytes) {
            Ok(status) => Ok(status),
            Err(reason) => Err(match reason {
                status::Error::Decode => Error::StatusDecode,
                status::Error::Undefined => Error::StatusUndefined,
            }),
        }?;

        // Done
        Ok(Self {
            status,
            meta: match Meta::from_header(&bytes) {
                Ok(meta) => Some(meta),
                Err(_) => None,
            },
            mime: match Mime::from_header(&bytes) {
                Ok(mime) => Some(mime),
                Err(_) => None,
            },
        })
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

    /// Get last header byte (until \r)
    fn end(bytes: &Bytes) -> Result<usize, Error> {
        for (offset, &byte) in bytes.iter().enumerate() {
            if byte == b'\r' {
                return Ok(offset);
            }
        }
        Err(Error::Format)
    }
}
