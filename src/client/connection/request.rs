pub mod error;
pub use error::Error;

// Local dependencies

use gio::NetworkAddress;
use glib::{Bytes, Uri, UriHideFlags};

/// Single `Request` implementation for different protocols
pub enum Request {
    Gemini {
        uri: Uri,
    },
    Titan {
        uri: Uri,
        data: Bytes,
        /// MIME type is optional attribute by Titan protocol specification,
        /// but server MAY reject the request without `mime` value provided.
        mime: Option<String>,
        token: Option<String>,
    },
}

impl Request {
    // Getters

    /// Generate header string for `Self`
    pub fn header(&self) -> String {
        match self {
            Self::Gemini { uri } => format!("{uri}\r\n"),
            Self::Titan {
                uri,
                data,
                mime,
                token,
            } => {
                let mut header = format!(
                    "{};size={}",
                    uri.to_string_partial(UriHideFlags::QUERY),
                    data.len()
                );
                if let Some(mime) = mime {
                    header.push_str(&format!(";mime={mime}"));
                }
                if let Some(token) = token {
                    header.push_str(&format!(";token={token}"));
                }
                if let Some(query) = uri.query() {
                    header.push_str(&format!("?{query}"));
                }
                header.push_str("\r\n");
                header
            }
        }
    }

    /// Get reference to `Self` [Uri](https://docs.gtk.org/glib/struct.Uri.html)
    pub fn uri(&self) -> &Uri {
        match self {
            Self::Gemini { uri } => uri,
            Self::Titan { uri, .. } => uri,
        }
    }

    /// Get [NetworkAddress](https://docs.gtk.org/gio/class.NetworkAddress.html) for `Self`
    pub fn to_network_address(&self, default_port: u16) -> Result<NetworkAddress, Error> {
        match crate::gio::network_address::from_uri(self.uri(), default_port) {
            Ok(network_address) => Ok(network_address),
            Err(e) => Err(Error::NetworkAddress(e)),
        }
    }
}

#[test]
fn test_gemini_header() {
    use glib::UriFlags;

    const REQUEST: &str = "gemini://geminiprotocol.net/";

    assert_eq!(
        Request::Gemini {
            uri: Uri::parse(REQUEST, UriFlags::NONE).unwrap()
        }
        .header(),
        format!("{REQUEST}\r\n")
    );
}

#[test]
fn test_titan_header() {
    use glib::UriFlags;

    const DATA: &[u8] = &[1, 2, 3];
    const MIME: &str = "plain/text";
    const TOKEN: &str = "token";

    assert_eq!(
        Request::Titan {
            uri: Uri::parse(
                "titan://geminiprotocol.net/raw/path?key=value",
                UriFlags::NONE
            )
            .unwrap(),
            data: Bytes::from(DATA),
            mime: Some(MIME.to_string()),
            token: Some(TOKEN.to_string())
        }
        .header(),
        format!(
            "titan://geminiprotocol.net/raw/path;size={};mime={MIME};token={TOKEN}?key=value\r\n",
            DATA.len(),
        )
    );
}
