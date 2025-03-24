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
        let header = Header::parse(buffer).map_err(|e| Error::Header(e))?;
        Ok(Self {
            content: buffer.get(header.len() + 1..).map(|v| v.to_vec()),
            header,
        })
    }
}
