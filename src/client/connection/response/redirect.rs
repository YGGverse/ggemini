pub mod error;
pub use error::Error;

use glib::GStringPtr;

const TEMPORARY: u8 = 30;
const PERMANENT: u8 = 31;

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
                Self::Permanent { target } => format!("Permanent redirection to `{target}`"),
                Self::Temporary { target } => format!("Temporary redirection to `{target}`"),
            }
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
fn test_from_str() {
    use std::str::FromStr;

    let temporary = Redirect::from_str("30 /uri\r\n").unwrap();
    assert_eq!(temporary.target(), "/uri");
    assert_eq!(temporary.to_code(), TEMPORARY);

    let permanent = Redirect::from_str("31 /uri\r\n").unwrap();
    assert_eq!(permanent.target(), "/uri");
    assert_eq!(permanent.to_code(), PERMANENT);
}
