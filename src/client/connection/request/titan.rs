use glib::{Bytes, Uri};

/// [Titan](gemini://transjovian.org/titan/page/The%20Titan%20Specification) protocol enum object for `Request`
pub struct Titan {
    pub uri: Uri,
    pub size: usize,
    pub mime: String,
    pub token: Option<String>,
    pub data: Vec<u8>,
}

impl Titan {
    // Constructors

    /// Build valid new `Self`
    pub fn build(
        uri: Uri,
        size: usize,
        mime: String,
        token: Option<String>,
        data: Vec<u8>,
    ) -> Self {
        Self {
            uri,
            size,
            mime,
            token,
            data,
        } // @TODO validate
    }

    // Getters

    /// Copy `Self` to [Bytes](https://docs.gtk.org/glib/struct.Bytes.html)
    pub fn to_bytes(&self) -> Bytes {
        // Build header
        let mut header = format!("{};size={};mime={}", self.uri, self.size, self.mime);
        if let Some(ref token) = self.token {
            header.push_str(&format!(";token={token}"));
        }
        header.push_str("\r\n");

        let header_bytes = header.into_bytes();

        // Build request
        let mut bytes: Vec<u8> = Vec::with_capacity(self.size + header_bytes.len());
        bytes.extend(header_bytes);
        bytes.extend(&self.data);

        // Wrap result
        Bytes::from(&bytes)
    }
}
