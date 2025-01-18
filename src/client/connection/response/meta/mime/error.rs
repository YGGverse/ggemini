#[derive(Debug)]
pub enum Error {
    Decode(std::str::Utf8Error),
    Protocol,
    Undefined,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Decode(e) => {
                write!(f, "Decode error: {e}")
            }
            Self::Protocol => {
                write!(f, "Protocol error")
            }
            Self::Undefined => {
                write!(f, "MIME type undefined")
            }
        }
    }
}
