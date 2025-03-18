pub mod error;
pub use error::Error;

const DEFAULT: (u8, &str) = (50, "Unspecified");
const NOT_FOUND: (u8, &str) = (51, "Not found");
const GONE: (u8, &str) = (52, "Gone");
const PROXY_REQUEST_REFUSED: (u8, &str) = (53, "Proxy request refused");
const BAD_REQUEST: (u8, &str) = (59, "bad-request");

/// https://geminiprotocol.net/docs/protocol-specification.gmi#permanent-failure
pub enum Permanent {
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-50
    Default {
        header: String,
        message: Option<String>,
    },
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-51-not-found
    NotFound {
        header: String,
        message: Option<String>,
    },
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-52-gone
    Gone {
        header: String,
        message: Option<String>,
    },
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-53-proxy-request-refused
    ProxyRequestRefused {
        header: String,
        message: Option<String>,
    },
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-59-bad-request
    BadRequest {
        header: String,
        message: Option<String>,
    },
}

impl Permanent {
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
            Self::NotFound { .. } => NOT_FOUND,
            Self::Gone { .. } => GONE,
            Self::ProxyRequestRefused { .. } => PROXY_REQUEST_REFUSED,
            Self::BadRequest { .. } => BAD_REQUEST,
        }
        .0
    }

    pub fn header(&self) -> &str {
        match self {
            Self::Default { header, .. }
            | Self::NotFound { header, .. }
            | Self::Gone { header, .. }
            | Self::ProxyRequestRefused { header, .. }
            | Self::BadRequest { header, .. } => header,
        }
        .as_str()
    }

    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Default { message, .. }
            | Self::NotFound { message, .. }
            | Self::Gone { message, .. }
            | Self::ProxyRequestRefused { message, .. }
            | Self::BadRequest { message, .. } => message,
        }
        .as_deref()
    }
}

impl std::fmt::Display for Permanent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Default { .. } => DEFAULT,
                Self::NotFound { .. } => NOT_FOUND,
                Self::Gone { .. } => GONE,
                Self::ProxyRequestRefused { .. } => PROXY_REQUEST_REFUSED,
                Self::BadRequest { .. } => BAD_REQUEST,
            }
            .1
        )
    }
}

impl std::str::FromStr for Permanent {
    type Err = Error;
    fn from_str(header: &str) -> Result<Self, Self::Err> {
        if let Some(postfix) = header.strip_prefix("50") {
            return Ok(Self::Default {
                header: header.to_string(),
                message: message(postfix),
            });
        }
        if let Some(postfix) = header.strip_prefix("51") {
            return Ok(Self::NotFound {
                header: header.to_string(),
                message: message(postfix),
            });
        }
        if let Some(postfix) = header.strip_prefix("52") {
            return Ok(Self::Gone {
                header: header.to_string(),
                message: message(postfix),
            });
        }
        if let Some(postfix) = header.strip_prefix("53") {
            return Ok(Self::ProxyRequestRefused {
                header: header.to_string(),
                message: message(postfix),
            });
        }
        if let Some(postfix) = header.strip_prefix("59") {
            return Ok(Self::BadRequest {
                header: header.to_string(),
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

    // 50
    let default = Permanent::from_str("50 Message\r\n").unwrap();
    assert_eq!(default.message(), Some("Message"));
    assert_eq!(default.to_code(), DEFAULT.0);
    assert_eq!(default.to_string(), DEFAULT.1);

    let default = Permanent::from_str("50\r\n").unwrap();
    assert_eq!(default.message(), None);
    assert_eq!(default.to_code(), DEFAULT.0);
    assert_eq!(default.to_string(), DEFAULT.1);

    // 51
    let not_found = Permanent::from_str("51 Message\r\n").unwrap();
    assert_eq!(not_found.message(), Some("Message"));
    assert_eq!(not_found.to_code(), NOT_FOUND.0);
    assert_eq!(not_found.to_string(), NOT_FOUND.1);

    let not_found = Permanent::from_str("51\r\n").unwrap();
    assert_eq!(not_found.message(), None);
    assert_eq!(not_found.to_code(), NOT_FOUND.0);
    assert_eq!(not_found.to_string(), NOT_FOUND.1);

    // 52
    let gone = Permanent::from_str("52 Message\r\n").unwrap();
    assert_eq!(gone.message(), Some("Message"));
    assert_eq!(gone.to_code(), GONE.0);
    assert_eq!(gone.to_string(), GONE.1);

    let gone = Permanent::from_str("52\r\n").unwrap();
    assert_eq!(gone.message(), None);
    assert_eq!(gone.to_code(), GONE.0);
    assert_eq!(gone.to_string(), GONE.1);

    // 53
    let proxy_request_refused = Permanent::from_str("53 Message\r\n").unwrap();
    assert_eq!(proxy_request_refused.message(), Some("Message"));
    assert_eq!(proxy_request_refused.to_code(), PROXY_REQUEST_REFUSED.0);
    assert_eq!(proxy_request_refused.to_string(), PROXY_REQUEST_REFUSED.1);

    let proxy_request_refused = Permanent::from_str("53\r\n").unwrap();
    assert_eq!(proxy_request_refused.message(), None);
    assert_eq!(proxy_request_refused.to_code(), PROXY_REQUEST_REFUSED.0);
    assert_eq!(proxy_request_refused.to_string(), PROXY_REQUEST_REFUSED.1);

    // 59
    let bad_request = Permanent::from_str("59 Message\r\n").unwrap();
    assert_eq!(bad_request.message(), Some("Message"));
    assert_eq!(bad_request.to_code(), BAD_REQUEST.0);
    assert_eq!(bad_request.to_string(), BAD_REQUEST.1);

    let bad_request = Permanent::from_str("59\r\n").unwrap();
    assert_eq!(bad_request.message(), None);
    assert_eq!(bad_request.to_code(), BAD_REQUEST.0);
    assert_eq!(bad_request.to_string(), BAD_REQUEST.1);
}
