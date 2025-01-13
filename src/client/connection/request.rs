pub mod error;
pub mod gemini;
pub mod titan;

pub use error::Error;
pub use gemini::Gemini;
pub use titan::Titan;

use gio::NetworkAddress;
use glib::Uri;

/// Single `Request` implementation for different protocols
pub enum Request {
    Gemini(Gemini),
    Titan(Titan),
}

impl Request {
    // Constructors

    /// Create new `Self` for [Gemini protocol](https://geminiprotocol.net)
    pub fn gemini(uri: Uri) -> Self {
        Self::Gemini(Gemini { uri })
    }

    /// Create new `Self` for [Titan protocol](gemini://transjovian.org/titan/page/The%20Titan%20Specification)
    pub fn titan(uri: Uri, data: Vec<u8>, mime: Option<String>, token: Option<String>) -> Self {
        Self::Titan(Titan {
            uri,
            data,
            mime,
            token,
        })
    }

    // Getters

    /// Get [NetworkAddress](https://docs.gtk.org/gio/class.NetworkAddress.html) for `Self`
    pub fn to_network_address(&self, default_port: u16) -> Result<NetworkAddress, Error> {
        match crate::gio::network_address::from_uri(
            &match self {
                Self::Gemini(ref request) => request.uri.clone(),
                Self::Titan(ref request) => request.uri.clone(),
            },
            default_port,
        ) {
            Ok(network_address) => Ok(network_address),
            Err(e) => Err(Error::NetworkAddress(e)),
        }
    }
}
