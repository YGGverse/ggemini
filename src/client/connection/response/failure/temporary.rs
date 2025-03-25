pub mod cgi_error;
pub mod default;
pub mod error;
pub mod proxy_error;
pub mod server_unavailable;
pub mod slow_down;

pub use cgi_error::CgiError;
pub use default::Default;
pub use error::Error;
pub use proxy_error::ProxyError;
pub use server_unavailable::ServerUnavailable;
pub use slow_down::SlowDown;

const CODE: u8 = b'4';

/// https://geminiprotocol.net/docs/protocol-specification.gmi#temporary-failure
pub enum Temporary {
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-40
    Default(Default),
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-41-server-unavailable
    ServerUnavailable(ServerUnavailable),
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-42-cgi-error
    CgiError(CgiError),
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-43-proxy-error
    ProxyError(ProxyError),
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-44-slow-down
    SlowDown(SlowDown),
}

impl Temporary {
    // Constructors

    /// Create new `Self` from buffer include header bytes
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        match buffer.first() {
            Some(b) => match *b {
                CODE => match buffer.get(1) {
                    Some(b) => match *b {
                        b'0' => Ok(Self::Default(
                            Default::from_utf8(buffer).map_err(Error::Default)?,
                        )),
                        b'1' => Ok(Self::ServerUnavailable(
                            ServerUnavailable::from_utf8(buffer)
                                .map_err(Error::ServerUnavailable)?,
                        )),
                        b'2' => Ok(Self::CgiError(
                            CgiError::from_utf8(buffer).map_err(Error::CgiError)?,
                        )),
                        b'3' => Ok(Self::ProxyError(
                            ProxyError::from_utf8(buffer).map_err(Error::ProxyError)?,
                        )),
                        b'4' => Ok(Self::SlowDown(
                            SlowDown::from_utf8(buffer).map_err(Error::SlowDown)?,
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

    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Default(default) => default.message(),
            Self::ServerUnavailable(server_unavailable) => server_unavailable.message(),
            Self::CgiError(cgi_error) => cgi_error.message(),
            Self::ProxyError(proxy_error) => proxy_error.message(),
            Self::SlowDown(slow_down) => slow_down.message(),
        }
    }

    /// Get optional message for `Self`
    /// * if the optional message not provided by the server, return children `DEFAULT_MESSAGE`
    pub fn message_or_default(&self) -> &str {
        match self {
            Self::Default(default) => default.message_or_default(),
            Self::ServerUnavailable(server_unavailable) => server_unavailable.message_or_default(),
            Self::CgiError(cgi_error) => cgi_error.message_or_default(),
            Self::ProxyError(proxy_error) => proxy_error.message_or_default(),
            Self::SlowDown(slow_down) => slow_down.message_or_default(),
        }
    }

    /// Get header string of `Self`
    pub fn as_str(&self) -> &str {
        match self {
            Self::Default(default) => default.as_str(),
            Self::ServerUnavailable(server_unavailable) => server_unavailable.as_str(),
            Self::CgiError(cgi_error) => cgi_error.as_str(),
            Self::ProxyError(proxy_error) => proxy_error.as_str(),
            Self::SlowDown(slow_down) => slow_down.as_str(),
        }
    }

    /// Get header bytes of `Self`
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Default(default) => default.as_bytes(),
            Self::ServerUnavailable(server_unavailable) => server_unavailable.as_bytes(),
            Self::CgiError(cgi_error) => cgi_error.as_bytes(),
            Self::ProxyError(proxy_error) => proxy_error.as_bytes(),
            Self::SlowDown(slow_down) => slow_down.as_bytes(),
        }
    }
}

#[test]
fn test() {
    fn t(source: String, message: Option<&str>) {
        let b = source.as_bytes();
        let i = Temporary::from_utf8(b).unwrap();
        assert_eq!(i.message(), message);
        assert_eq!(i.as_str(), source);
        assert_eq!(i.as_bytes(), b);
    }
    for code in [40, 41, 42, 43, 44] {
        t(format!("{code} Message\r\n"), Some("Message"));
        t(format!("{code}\r\n"), None);
    }
}
