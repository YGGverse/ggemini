use glib::{Bytes, Uri, UriHideFlags};

/// [Titan](gemini://transjovian.org/titan/page/The%20Titan%20Specification) protocol enum object for `Request`
pub struct Titan {
    pub uri: Uri,
    pub data: Bytes,
    pub mime: Option<String>,
    pub token: Option<String>,
}

impl Titan {
    // Getters

    /// Get header string for `Self`
    pub fn header(&self) -> String {
        let mut header = format!(
            "{};size={}",
            self.uri.to_string_partial(UriHideFlags::QUERY),
            self.data.len()
        );
        if let Some(ref mime) = self.mime {
            header.push_str(&format!(";mime={mime}"));
        }
        if let Some(ref token) = self.token {
            header.push_str(&format!(";token={token}"));
        }
        if let Some(query) = self.uri.query() {
            header.push_str(&format!("?{query}"));
        }
        header.push_str("\r\n");
        header
    }
}
