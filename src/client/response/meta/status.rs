//! Parser and holder tools for
//! [Status code](https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes)

pub mod error;
pub use error::Error;

use glib::GString;

/// Holder for [status code](https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes)
#[derive(Debug)]
pub enum Status {
    // Input
    Input = 10,
    SensitiveInput = 11,
    // Success
    Success = 20,
    // Redirect
    Redirect = 30,
    PermanentRedirect = 31,
    // Temporary failure
    TemporaryFailure = 40,
    ServerUnavailable = 41,
    CgiError = 42,
    ProxyError = 43,
    SlowDown = 44,
    // Permanent failure
    PermanentFailure = 50,
    NotFound = 51,
    ResourceGone = 52,
    ProxyRequestRefused = 53,
    BadRequest = 59,
    // Client certificates
    CertificateRequest = 60,
    CertificateUnauthorized = 61,
    CertificateInvalid = 62,
}

impl Status {
    /// Create new `Self` from UTF-8 buffer
    ///
    /// * includes `Self::from_string` parser, it means that given buffer should contain some **header**
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        match buffer.get(0..2) {
            Some(value) => match GString::from_utf8(value.to_vec()) {
                Ok(string) => Self::from_string(string.as_str()),
                Err(e) => Err(Error::Decode(e)),
            },
            None => Err(Error::Protocol),
        }
    }

    /// Create new `Self` from string that includes **header**
    pub fn from_string(code: &str) -> Result<Self, Error> {
        match code {
            // Input
            "10" => Ok(Self::Input),
            "11" => Ok(Self::SensitiveInput),
            // Success
            "20" => Ok(Self::Success),
            // Redirect
            "30" => Ok(Self::Redirect),
            "31" => Ok(Self::PermanentRedirect),
            // Temporary failure
            "40" => Ok(Self::TemporaryFailure),
            "41" => Ok(Self::ServerUnavailable),
            "42" => Ok(Self::CgiError),
            "43" => Ok(Self::ProxyError),
            "44" => Ok(Self::SlowDown),
            // Permanent failure
            "50" => Ok(Self::PermanentFailure),
            "51" => Ok(Self::NotFound),
            "52" => Ok(Self::ResourceGone),
            "53" => Ok(Self::ProxyRequestRefused),
            "59" => Ok(Self::BadRequest),
            // Client certificates
            "60" => Ok(Self::CertificateRequest),
            "61" => Ok(Self::CertificateUnauthorized),
            "62" => Ok(Self::CertificateInvalid),
            _ => Err(Error::Undefined),
        }
    }
}
