pub mod error;
pub mod permanent;
pub mod temporary;

pub use error::{Error, UriError};
pub use permanent::Permanent;
pub use temporary::Temporary;

// Local dependencies

use glib::{Uri, UriFlags};

const CODE: u8 = b'3';

/// [Redirection](https://geminiprotocol.net/docs/protocol-specification.gmi#redirection) statuses
pub enum Redirect {
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-30-temporary-redirection
    Temporary(Temporary),
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-31-permanent-redirection
    Permanent(Permanent),
}

impl Redirect {
    // Constructors

    /// Create new `Self` from buffer include header bytes
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        match buffer.first() {
            Some(b) => match *b {
                CODE => match buffer.get(1) {
                    Some(b) => match *b {
                        b'0' => Ok(Self::Temporary(
                            Temporary::from_utf8(buffer).map_err(Error::Temporary)?,
                        )),
                        b'1' => Ok(Self::Permanent(
                            Permanent::from_utf8(buffer).map_err(Error::Permanent)?,
                        )),
                        b => Err(Error::SecondByte(b)),
                    },
                    None => Err(Error::UndefinedSecondByte),
                },
                b => Err(Error::FirstByte(b)),
            },
            None => Err(Error::UndefinedFirstByte),
        }
    }

    // Getters

    pub fn target(&self) -> Result<&str, Error> {
        match self {
            Self::Temporary(temporary) => temporary.target().map_err(Error::Temporary),
            Self::Permanent(permanent) => permanent.target().map_err(Error::Permanent),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Temporary(temporary) => temporary.as_str(),
            Self::Permanent(permanent) => permanent.as_str(),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Temporary(temporary) => temporary.as_bytes(),
            Self::Permanent(permanent) => permanent.as_bytes(),
        }
    }

    pub fn uri(&self, base: &Uri) -> Result<Uri, Error> {
        match self {
            Self::Temporary(temporary) => temporary.uri(base).map_err(Error::Temporary),
            Self::Permanent(permanent) => permanent.uri(base).map_err(Error::Permanent),
        }
    }
}

// Tools

/// Resolve [specification-compatible](https://geminiprotocol.net/docs/protocol-specification.gmi#redirection),
/// absolute [Uri](https://docs.gtk.org/glib/struct.Uri.html) for `target` using `base`
/// * fragment implementation uncompleted @TODO
fn uri(target: &str, base: &Uri) -> Result<Uri, UriError> {
    match Uri::build(
        UriFlags::NONE,
        base.scheme().as_str(),
        None, // unexpected
        base.host().as_deref(),
        base.port(),
        base.path().as_str(),
        // > If a server sends a redirection in response to a request with a query string,
        // > the client MUST NOT apply the query string to the new location
        None,
        // > A server SHOULD NOT include fragments in redirections,
        // > but if one is given, and a client already has a fragment it could apply (from the original URI),
        // > it is up to the client which fragment to apply.
        None, // @TODO
    )
    .parse_relative(
        &{
            // URI started with double slash yet not supported by Glib function
            // https://datatracker.ietf.org/doc/html/rfc3986#section-4.2
            let t = target;
            match t.strip_prefix("//") {
                Some(p) => {
                    let postfix = p.trim_start_matches(":");
                    format!(
                        "{}://{}",
                        base.scheme(),
                        if postfix.is_empty() {
                            match base.host() {
                                Some(h) => format!("{h}/"),
                                None => return Err(UriError::BaseHost),
                            }
                        } else {
                            postfix.to_string()
                        }
                    )
                }
                None => t.to_string(),
            }
        },
        UriFlags::NONE,
    ) {
        Ok(absolute) => Ok(absolute),
        Err(e) => Err(UriError::ParseRelative(e)),
    }
}

#[test]
fn test() {
    /// Test common assertion rules
    fn t(base: &Uri, source: &str, target: &str) {
        let b = source.as_bytes();
        let r = Redirect::from_utf8(b).unwrap();
        assert!(r.uri(base).is_ok_and(|u| u.to_string() == target));
        assert_eq!(r.as_str(), source);
        assert_eq!(r.as_bytes(), b);
    }
    // common base
    let base = Uri::build(
        UriFlags::NONE,
        "gemini",
        None,
        Some("geminiprotocol.net"),
        -1,
        "/path/",
        Some("query"),
        Some("fragment"),
    );
    // codes test
    t(
        &base,
        "30 gemini://geminiprotocol.net/path\r\n",
        "gemini://geminiprotocol.net/path",
    );
    t(
        &base,
        "31 gemini://geminiprotocol.net/path\r\n",
        "gemini://geminiprotocol.net/path",
    );
    // relative test
    t(
        &base,
        "31 path\r\n",
        "gemini://geminiprotocol.net/path/path",
    );
    t(
        &base,
        "31 //geminiprotocol.net\r\n",
        "gemini://geminiprotocol.net",
    );
    t(
        &base,
        "31 //geminiprotocol.net/path\r\n",
        "gemini://geminiprotocol.net/path",
    );
    t(&base, "31 /path\r\n", "gemini://geminiprotocol.net/path");
    t(&base, "31 //:\r\n", "gemini://geminiprotocol.net/");
    t(&base, "31 //\r\n", "gemini://geminiprotocol.net/");
    t(&base, "31 /\r\n", "gemini://geminiprotocol.net/");
    t(&base, "31 ../\r\n", "gemini://geminiprotocol.net/");
    t(&base, "31 ..\r\n", "gemini://geminiprotocol.net/");
}
