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

#[test]
fn header() {
    use super::{super::Request, Titan};
    use glib::UriFlags;

    const DATA: &[u8] = &[1, 2, 3];
    const MIME: &str = "plain/text";
    const TOKEN: &str = "token";

    assert_eq!(
        Request::Titan(Titan {
            uri: Uri::parse(
                "titan://geminiprotocol.net/raw/path?key=value",
                UriFlags::NONE
            )
            .unwrap(),
            data: Bytes::from(DATA),
            mime: Some(MIME.to_string()),
            token: Some(TOKEN.to_string())
        })
        .header(),
        format!(
            "titan://geminiprotocol.net/raw/path;size={};mime={MIME};token={TOKEN}?key=value\r\n",
            DATA.len(),
        )
    );
}
