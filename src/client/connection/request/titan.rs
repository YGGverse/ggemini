use glib::{Bytes, Uri};

/// [Titan](gemini://transjovian.org/titan/page/The%20Titan%20Specification) protocol enum object for `Request`
pub struct Titan {
    pub uri: Uri,
    pub mime: String,
    pub token: Option<String>,
    pub data: Vec<u8>,
}

impl Titan {
    // Constructors

    /// Build valid new `Self`
    pub fn build(uri: Uri, mime: String, token: Option<String>, data: Vec<u8>) -> Self {
        Self {
            uri,
            mime,
            token,
            data,
        } // @TODO validate
    }

    // Getters

    /// Copy `Self` to [Bytes](https://docs.gtk.org/glib/struct.Bytes.html)
    pub fn to_bytes(&self) -> Bytes {
        // Calculate data size
        let size = self.data.len();

        // Build header
        let mut header = format!("{};size={size};mime={}", self.uri, self.mime);
        if let Some(ref token) = self.token {
            header.push_str(&format!(";token={token}"));
        }
        header.push_str("\r\n");

        // Build request
        let mut bytes: Vec<u8> = Vec::with_capacity(size + 1024); // @TODO
        bytes.extend(header.into_bytes());
        bytes.extend(&self.data);

        // Wrap result
        Bytes::from(&bytes)
    }
}
