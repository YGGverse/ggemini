pub mod error;
pub use error::Error;

const REQUIRED: (u8, &str) = (60, "Certificate required");
const NOT_AUTHORIZED: (u8, &str) = (61, "Certificate not authorized");
const NOT_VALID: (u8, &str) = (62, "Certificate not valid");

/// 6* status code group
/// https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates
pub enum Certificate {
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-60
    Required { message: Option<String> },
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-61-certificate-not-authorized
    NotAuthorized { message: Option<String> },
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-62-certificate-not-valid
    NotValid { message: Option<String> },
}

impl Certificate {
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
            Self::Required { .. } => REQUIRED,
            Self::NotAuthorized { .. } => NOT_AUTHORIZED,
            Self::NotValid { .. } => NOT_VALID,
        }
        .0
    }

    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Required { message } => message,
            Self::NotAuthorized { message } => message,
            Self::NotValid { message } => message,
        }
        .as_deref()
    }
}

impl std::fmt::Display for Certificate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Required { .. } => REQUIRED,
                Self::NotAuthorized { .. } => NOT_AUTHORIZED,
                Self::NotValid { .. } => NOT_VALID,
            }
            .1
        )
    }
}

impl std::str::FromStr for Certificate {
    type Err = Error;
    fn from_str(header: &str) -> Result<Self, Self::Err> {
        if let Some(postfix) = header.strip_prefix("60") {
            return Ok(Self::Required {
                message: message(postfix),
            });
        }
        if let Some(postfix) = header.strip_prefix("61") {
            return Ok(Self::NotAuthorized {
                message: message(postfix),
            });
        }
        if let Some(postfix) = header.strip_prefix("62") {
            return Ok(Self::NotValid {
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

    let required = Certificate::from_str("60 Message\r\n").unwrap();

    assert_eq!(required.message(), Some("Message"));
    assert_eq!(required.to_code(), REQUIRED.0);
    assert_eq!(required.to_string(), REQUIRED.1);

    let required = Certificate::from_str("60\r\n").unwrap();

    assert_eq!(required.message(), None);
    assert_eq!(required.to_code(), REQUIRED.0);
    assert_eq!(required.to_string(), REQUIRED.1);
}
