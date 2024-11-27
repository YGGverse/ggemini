pub mod error;
pub mod scope;

pub use error::Error;
pub use scope::Scope;

use gio::{prelude::TlsCertificateExt, TlsCertificate};
use glib::DateTime;

pub struct Certificate {
    tls_certificate: TlsCertificate,
}

impl Certificate {
    // Constructors

    /// Create new `Self`
    pub fn from_pem(pem: &str) -> Result<Self, Error> {
        Ok(Self {
            tls_certificate: match TlsCertificate::from_pem(&pem) {
                Ok(tls_certificate) => {
                    // Validate expiration time
                    match DateTime::now_local() {
                        Ok(now_local) => {
                            match tls_certificate.not_valid_after() {
                                Some(not_valid_after) => {
                                    if now_local > not_valid_after {
                                        return Err(Error::Expired(not_valid_after));
                                    }
                                }
                                None => return Err(Error::ValidAfter),
                            }
                            match tls_certificate.not_valid_before() {
                                Some(not_valid_before) => {
                                    if now_local < not_valid_before {
                                        return Err(Error::Inactive(not_valid_before));
                                    }
                                }
                                None => return Err(Error::ValidBefore),
                            }
                        }
                        Err(_) => return Err(Error::DateTime),
                    }

                    // Success
                    tls_certificate
                }
                Err(reason) => return Err(Error::Decode(reason)),
            },
        })
    }
}
