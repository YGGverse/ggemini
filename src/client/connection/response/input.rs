pub mod error;
pub use error::Error;

const DEFAULT: (u8, &str) = (10, "Input");
const SENSITIVE: (u8, &str) = (11, "Sensitive input");

pub enum Input {
    Default {
        header: String,
        message: Option<String>,
    },
    Sensitive {
        header: String,
        message: Option<String>,
    },
}

impl Input {
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
            Self::Sensitive { .. } => SENSITIVE,
        }
        .0
    }

    pub fn header(&self) -> &str {
        match self {
            Self::Default { header, .. } | Self::Sensitive { header, .. } => header,
        }
        .as_str()
    }

    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Default { message, .. } | Self::Sensitive { message, .. } => message,
        }
        .as_deref()
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Default { .. } => DEFAULT,
                Self::Sensitive { .. } => SENSITIVE,
            }
            .1
        )
    }
}

impl std::str::FromStr for Input {
    type Err = Error;
    fn from_str(header: &str) -> Result<Self, Self::Err> {
        if let Some(postfix) = header.strip_prefix("10") {
            return Ok(Self::Default {
                header: header.to_string(),
                message: message(postfix),
            });
        }
        if let Some(postfix) = header.strip_prefix("11") {
            return Ok(Self::Sensitive {
                header: header.to_string(),
                message: message(postfix),
            });
        }
        Err(Error::Protocol)
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

    // 10
    let default = Input::from_str("10 Default\r\n").unwrap();
    assert_eq!(default.message(), Some("Default"));
    assert_eq!(default.to_code(), DEFAULT.0);
    assert_eq!(default.to_string(), DEFAULT.1);

    let default = Input::from_str("10\r\n").unwrap();
    assert_eq!(default.message(), None);
    assert_eq!(default.to_code(), DEFAULT.0);
    assert_eq!(default.to_string(), DEFAULT.1);

    // 11
    let sensitive = Input::from_str("11 Sensitive\r\n").unwrap();
    assert_eq!(sensitive.message(), Some("Sensitive"));
    assert_eq!(sensitive.to_code(), SENSITIVE.0);
    assert_eq!(sensitive.to_string(), SENSITIVE.1);

    let sensitive = Input::from_str("11\r\n").unwrap();
    assert_eq!(sensitive.message(), None);
    assert_eq!(sensitive.to_code(), SENSITIVE.0);
    assert_eq!(sensitive.to_string(), SENSITIVE.1);
}
