pub mod error;
pub use error::Error;

const DEFAULT: (u8, &str) = (20, "Success");

pub enum Success {
    Default { mime: String },
    // reserved for 2* codes
}

impl Success {
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
            Self::Default { .. } => DEFAULT.0,
        }
    }

    // Getters

    pub fn mime(&self) -> &str {
        match self {
            Self::Default { mime } => mime,
        }
    }
}

impl std::fmt::Display for Success {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Default { .. } => DEFAULT.1,
            }
        )
    }
}

impl std::str::FromStr for Success {
    type Err = Error;
    fn from_str(header: &str) -> Result<Self, Self::Err> {
        use glib::{Regex, RegexCompileFlags, RegexMatchFlags};

        match Regex::split_simple(
            r"^20\s([^\/]+\/[^\s;]+)",
            header,
            RegexCompileFlags::DEFAULT,
            RegexMatchFlags::DEFAULT,
        )
        .get(1)
        {
            Some(mime) => {
                let mime = mime.trim();
                if mime.is_empty() {
                    Err(Error::Mime)
                } else {
                    Ok(Self::Default {
                        mime: mime.to_lowercase(),
                    })
                }
            }
            None => Err(Error::Protocol),
        }
    }
}

#[test]
fn test_from_str() {
    use std::str::FromStr;

    let default = Success::from_str("20 text/gemini; charset=utf-8; lang=en\r\n").unwrap();

    assert_eq!(default.mime(), "text/gemini");
    assert_eq!(default.to_code(), DEFAULT.0);
    assert_eq!(default.to_string(), DEFAULT.1);
}
