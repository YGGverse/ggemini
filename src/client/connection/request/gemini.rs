use glib::Uri;

/// [Gemini](https://geminiprotocol.net/docs/protocol-specification.gmi) protocol enum object for `Request`
pub struct Gemini {
    pub uri: Uri,
}

impl Gemini {
    // Getters

    /// Get header string for `Self`
    pub fn header(&self) -> String {
        format!("{}\r\n", self.uri)
    }
}
