pub mod error;
pub use error::Error;

const DEFAULT: (u8, &str) = (40, "Unspecified");
const SERVER_UNAVAILABLE: (u8, &str) = (41, "Server unavailable");
const CGI_ERROR: (u8, &str) = (42, "CGI error");
const PROXY_ERROR: (u8, &str) = (43, "Proxy error");
const SLOW_DOWN: (u8, &str) = (44, "Slow down");

/// https://geminiprotocol.net/docs/protocol-specification.gmi#temporary-failure
pub enum Temporary {
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-40
    Default { message: Option<String> },
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-41-server-unavailable
    ServerUnavailable { message: Option<String> },
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-42-cgi-error
    CgiError { message: Option<String> },
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-43-proxy-error
    ProxyError { message: Option<String> },
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-44-slow-down
    SlowDown { message: Option<String> },
}

impl Temporary {
    // Constructors

    /// Create new `Self` from buffer include header bytes
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        use std::str::FromStr;
        match std::str::from_utf8(buffer) {
            Ok(header) => Self::from_str(header),
            Err(e) => Err(Error::Utf8Error(e)),
        }
    }

    // Getters

    pub fn to_code(&self) -> u8 {
        match self {
            Self::Default { .. } => DEFAULT,
            Self::ServerUnavailable { .. } => SERVER_UNAVAILABLE,
            Self::CgiError { .. } => CGI_ERROR,
            Self::ProxyError { .. } => PROXY_ERROR,
            Self::SlowDown { .. } => SLOW_DOWN,
        }
        .0
    }

    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Default { message } => message,
            Self::ServerUnavailable { message } => message,
            Self::CgiError { message } => message,
            Self::ProxyError { message } => message,
            Self::SlowDown { message } => message,
        }
        .as_deref()
    }
}

impl std::fmt::Display for Temporary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Default { .. } => DEFAULT,
                Self::ServerUnavailable { .. } => SERVER_UNAVAILABLE,
                Self::CgiError { .. } => CGI_ERROR,
                Self::ProxyError { .. } => PROXY_ERROR,
                Self::SlowDown { .. } => SLOW_DOWN,
            }
            .1
        )
    }
}

impl std::str::FromStr for Temporary {
    type Err = Error;
    fn from_str(header: &str) -> Result<Self, Self::Err> {
        if let Some(postfix) = header.strip_prefix("40") {
            return Ok(Self::Default {
                message: message(postfix),
            });
        }
        if let Some(postfix) = header.strip_prefix("41") {
            return Ok(Self::ServerUnavailable {
                message: message(postfix),
            });
        }
        if let Some(postfix) = header.strip_prefix("42") {
            return Ok(Self::CgiError {
                message: message(postfix),
            });
        }
        if let Some(postfix) = header.strip_prefix("43") {
            return Ok(Self::ProxyError {
                message: message(postfix),
            });
        }
        if let Some(postfix) = header.strip_prefix("44") {
            return Ok(Self::SlowDown {
                message: message(postfix),
            });
        }
        Err(Error::Code)
    }
}

// Tools

fn message(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

#[test]
fn test_from_str() {
    use std::str::FromStr;

    // 40
    let default = Temporary::from_str("40 Message\r\n").unwrap();
    assert_eq!(default.message(), Some("Message"));
    assert_eq!(default.to_code(), DEFAULT.0);
    assert_eq!(default.to_string(), DEFAULT.1);

    let default = Temporary::from_str("40\r\n").unwrap();
    assert_eq!(default.message(), None);
    assert_eq!(default.to_code(), DEFAULT.0);
    assert_eq!(default.to_string(), DEFAULT.1);

    // 41
    let server_unavailable = Temporary::from_str("41 Message\r\n").unwrap();
    assert_eq!(server_unavailable.message(), Some("Message"));
    assert_eq!(server_unavailable.to_code(), SERVER_UNAVAILABLE.0);
    assert_eq!(server_unavailable.to_string(), SERVER_UNAVAILABLE.1);

    let server_unavailable = Temporary::from_str("41\r\n").unwrap();
    assert_eq!(server_unavailable.message(), None);
    assert_eq!(server_unavailable.to_code(), SERVER_UNAVAILABLE.0);
    assert_eq!(server_unavailable.to_string(), SERVER_UNAVAILABLE.1);

    // 42
    let cgi_error = Temporary::from_str("42 Message\r\n").unwrap();
    assert_eq!(cgi_error.message(), Some("Message"));
    assert_eq!(cgi_error.to_code(), CGI_ERROR.0);
    assert_eq!(cgi_error.to_string(), CGI_ERROR.1);

    let cgi_error = Temporary::from_str("42\r\n").unwrap();
    assert_eq!(cgi_error.message(), None);
    assert_eq!(cgi_error.to_code(), CGI_ERROR.0);
    assert_eq!(cgi_error.to_string(), CGI_ERROR.1);

    // 43
    let proxy_error = Temporary::from_str("43 Message\r\n").unwrap();
    assert_eq!(proxy_error.message(), Some("Message"));
    assert_eq!(proxy_error.to_code(), PROXY_ERROR.0);
    assert_eq!(proxy_error.to_string(), PROXY_ERROR.1);

    let proxy_error = Temporary::from_str("43\r\n").unwrap();
    assert_eq!(proxy_error.message(), None);
    assert_eq!(proxy_error.to_code(), PROXY_ERROR.0);
    assert_eq!(proxy_error.to_string(), PROXY_ERROR.1);

    // 44
    let slow_down = Temporary::from_str("44 Message\r\n").unwrap();
    assert_eq!(slow_down.message(), Some("Message"));
    assert_eq!(slow_down.to_code(), SLOW_DOWN.0);
    assert_eq!(slow_down.to_string(), SLOW_DOWN.1);

    let slow_down = Temporary::from_str("44\r\n").unwrap();
    assert_eq!(slow_down.message(), None);
    assert_eq!(slow_down.to_code(), SLOW_DOWN.0);
    assert_eq!(slow_down.to_string(), SLOW_DOWN.1);
}
