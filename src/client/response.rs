pub mod body;
pub mod error;
pub mod header;

pub use body::Body;
pub use error::Error;
pub use header::Header;

use glib::Bytes;

pub struct Response {
    header: Header,
    body: Body,
}

impl Response {
    /// Create new `Self`
    pub fn new(header: Header, body: Body) -> Self {
        Self { header, body }
    }

    /// Construct from [Bytes](https://docs.gtk.org/glib/struct.Bytes.html)
    ///
    /// Useful for [Gio::InputStream](https://docs.gtk.org/gio/class.InputStream.html):
    /// * [read_bytes](https://docs.gtk.org/gio/method.InputStream.read_bytes.html)
    /// * [read_bytes_async](https://docs.gtk.org/gio/method.InputStream.read_bytes_async.html)
    pub fn from(bytes: &Bytes) -> Result<Self, Error> {
        let header = match Header::from_response(bytes) {
            Ok(result) => result,
            Err(_) => return Err(Error::Header),
        };

        let body = match Body::from_response(bytes) {
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
