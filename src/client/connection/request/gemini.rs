use glib::{Bytes, Uri};

/// [Gemini](https://geminiprotocol.net/docs/protocol-specification.gmi) protocol enum object for `Request`
pub struct Gemini {
    pub uri: Uri,
}

impl Gemini {
    // Getters

    /// Copy `Self` to [Bytes](https://docs.gtk.org/glib/struct.Bytes.html)
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(format!("{}\r\n", self.uri).as_bytes())
    }
}
