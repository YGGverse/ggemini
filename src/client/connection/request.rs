pub mod error;
pub mod gemini;
pub mod titan;

pub use error::Error;
pub use gemini::Gemini;
pub use titan::Titan;

// Local dependencies

use gio::NetworkAddress;
use glib::Uri;

/// Single `Request` implementation for different protocols
pub enum Request {
    Gemini(Gemini),
    Titan(Titan),
}

impl Request {
    // Getters

    /// Generate header string for `Self`
    pub fn header(&self) -> String {
        match self {
            Self::Gemini(ref this) => this.header(),
            Self::Titan(ref this) => this.header(),
        }
    }

    /// Get reference to `Self` [Uri](https://docs.gtk.org/glib/struct.Uri.html)
    pub fn uri(&self) -> &Uri {
        match self {
            Self::Gemini(ref this) => &this.uri,
            Self::Titan(ref this) => &this.uri,
        }
    }

    /// Get [NetworkAddress](https://docs.gtk.org/gio/class.NetworkAddress.html) for `Self`
    pub fn to_network_address(&self, default_port: u16) -> Result<NetworkAddress, Error> {
        match crate::gio::network_address::from_uri(self.uri(), default_port) {
            Ok(network_address) => Ok(network_address),
            Err(e) => Err(Error::NetworkAddress(e)),
        }
    }
}
