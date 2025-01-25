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

#[test]
fn header() {
    use super::{super::Request, Gemini};
    use glib::UriFlags;

    const REQUEST: &str = "gemini://geminiprotocol.net/";
    assert_eq!(
        Request::Gemini(Gemini {
            uri: Uri::parse(REQUEST, UriFlags::NONE).unwrap()
        })
        .header(),
        format!("{REQUEST}\r\n")
    );
}
