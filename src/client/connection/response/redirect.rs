pub mod error;
pub use error::Error;

use glib::{GStringPtr, Uri, UriFlags};

const TEMPORARY: (u8, &str) = (30, "Temporary redirect");
const PERMANENT: (u8, &str) = (31, "Permanent redirect");

pub enum Redirect {
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-30-temporary-redirection
    Temporary { target: String },
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-31-permanent-redirection
    Permanent { target: String },
}

impl Redirect {
    // Constructors

    /// Create new `Self` from buffer include header bytes
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        use std::str::FromStr;
        match std::str::from_utf8(buffer) {
            Ok(header) => Self::from_str(header),
            Err(e) => Err(Error::Utf8Error(e)),
        }
    }

    // Convertors

    pub fn to_code(&self) -> u8 {
        match self {
            Self::Permanent { .. } => PERMANENT,
            Self::Temporary { .. } => TEMPORARY,
        }
        .0
    }

    /// Resolve [specification-compatible](https://geminiprotocol.net/docs/protocol-specification.gmi#redirection),
    /// absolute [Uri](https://docs.gtk.org/glib/struct.Uri.html) for `target` using `base`
    /// * fragment implementation uncompleted @TODO
    pub fn to_uri(&self, base: &Uri) -> Result<Uri, Error> {
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
                let t = self.target();
                match t.strip_prefix("//") {
                    Some(p) => {
                        let postfix = p.trim_start_matches(":");
                        format!(
                            "{}://{}",
                            base.scheme(),
                            if postfix.is_empty() {
                                match base.host() {
                                    Some(h) => format!("{h}/"),
                                    None => return Err(Error::BaseHost),
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
            Err(e) => Err(Error::Uri(e)),
        }
    }

    // Getters

    pub fn target(&self) -> &str {
        match self {
            Self::Permanent { target } => target,
            Self::Temporary { target } => target,
        }
    }
}

impl std::fmt::Display for Redirect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Permanent { .. } => PERMANENT,
                Self::Temporary { .. } => TEMPORARY,
            }
            .1
        )
    }
}

impl std::str::FromStr for Redirect {
    type Err = Error;
    fn from_str(header: &str) -> Result<Self, Self::Err> {
        use glib::{Regex, RegexCompileFlags, RegexMatchFlags};

        let regex = Regex::split_simple(
            r"^3(\d)\s([^\r\n]+)",
            header,
            RegexCompileFlags::DEFAULT,
            RegexMatchFlags::DEFAULT,
        );

        match regex.get(1) {
            Some(code) => match code.as_str() {
                "0" => Ok(Self::Temporary {
                    target: target(regex.get(2))?,
                }),
                "1" => Ok(Self::Permanent {
                    target: target(regex.get(2))?,
                }),
                _ => todo!(),
            },
            None => Err(Error::Protocol),
        }
    }
}

fn target(value: Option<&GStringPtr>) -> Result<String, Error> {
    match value {
        Some(target) => {
            let target = target.trim();
            if target.is_empty() {
                Err(Error::Target)
            } else {
                Ok(target.to_string())
            }
        }
        None => Err(Error::Target),
    }
}

#[test]
fn test() {
    use std::str::FromStr;
    {
        let temporary = Redirect::from_str("30 /uri\r\n").unwrap();
        assert_eq!(temporary.target(), "/uri");
        assert_eq!(temporary.to_code(), TEMPORARY.0);
        assert_eq!(temporary.to_string(), TEMPORARY.1);

        let permanent = Redirect::from_str("31 /uri\r\n").unwrap();
        assert_eq!(permanent.target(), "/uri");
        assert_eq!(permanent.to_code(), PERMANENT.0);
        assert_eq!(permanent.to_string(), PERMANENT.1);
    }
    {
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
        assert_eq!(
            Redirect::from_str("30 /uri\r\n")
                .unwrap()
                .to_uri(&base)
                .unwrap()
                .to_string(),
            "gemini://geminiprotocol.net/uri"
        );
        assert_eq!(
            Redirect::from_str("30 uri\r\n")
                .unwrap()
                .to_uri(&base)
                .unwrap()
                .to_string(),
            "gemini://geminiprotocol.net/path/uri"
        );
        assert_eq!(
            Redirect::from_str("30 gemini://test.host/uri\r\n")
                .unwrap()
                .to_uri(&base)
                .unwrap()
                .to_string(),
            "gemini://test.host/uri"
        );
        assert_eq!(
            Redirect::from_str("30 //\r\n")
                .unwrap()
                .to_uri(&base)
                .unwrap()
                .to_string(),
            "gemini://geminiprotocol.net/"
        );
        assert_eq!(
            Redirect::from_str("30 //geminiprotocol.net/path\r\n")
                .unwrap()
                .to_uri(&base)
                .unwrap()
                .to_string(),
            "gemini://geminiprotocol.net/path"
        );
        assert_eq!(
            Redirect::from_str("30 //:\r\n")
                .unwrap()
                .to_uri(&base)
                .unwrap()
                .to_string(),
            "gemini://geminiprotocol.net/"
        );
    }
}
