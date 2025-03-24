pub mod error;
pub use error::Error;

pub struct Header(Vec<u8>);

impl Header {
    // Constructors

    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        if !buffer.starts_with(super::CODE) {
            return Err(Error::Code);
        }
        Ok(Self(
            crate::client::connection::response::header_bytes(buffer)
                .map_err(Error::Header)?
                .to_vec(),
        ))
    }

    // Getters

    /// Parse content type for `Self`
    pub fn mime(&self) -> Result<String, Error> {
        glib::Regex::split_simple(
            r"^\d{2}\s([^\/]+\/[^\s;]+)",
            std::str::from_utf8(&self.0).map_err(Error::Utf8Error)?,
            glib::RegexCompileFlags::DEFAULT,
            glib::RegexMatchFlags::DEFAULT,
        )
        .get(1)
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map_or(Err(Error::Mime), |s| Ok(s.to_lowercase()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}
