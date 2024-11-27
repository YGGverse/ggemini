pub mod error;
pub use error::Error;

use crate::DEFAULT_PORT;
use gio::NetworkAddress;
use glib::{GString, Uri, UriFlags, UriHideFlags};

/// Scope implement path prefix to apply TLS authorization for
/// * external validator MAY decline `Certificate` if `Scope` defined out of protocol range
/// * [read more](https://geminiprotocol.net/docs/protocol-specification.gmi#status-60)
pub struct Scope {
    uri: Uri,
}

impl Scope {
    // Constructors

    /// Create new `Self` for given `url`
    pub fn from_url(url: &str) -> Result<Self, Error> {
        match Uri::parse(url, UriFlags::NONE) {
            Ok(uri) => {
                if !uri.scheme().to_lowercase().contains("gemini") {
                    return Err(Error::Scheme);
                }

                if uri.host().is_none() {
                    return Err(Error::Host);
                }

                Ok(Self { uri })
            }
            Err(reason) => Err(Error::Uri(reason)),
        }
    }

    // Getters

    /// Get `Scope` string match [Specification](https://geminiprotocol.net/docs/protocol-specification.gmi#status-60)
    pub fn to_string(&self) -> GString {
        self.uri
            .to_string_partial(UriHideFlags::QUERY | UriHideFlags::FRAGMENT)
    }

    /// Get [NetworkAddress](https://docs.gtk.org/gio/class.NetworkAddress.html)
    /// implement [SocketConnectable](https://docs.gtk.org/gio/iface.SocketConnectable.html) interface
    /// * useful as [SNI](https://geminiprotocol.net/docs/protocol-specification.gmi#server-name-indication) in TLS context
    pub fn to_network_address(&self) -> Result<NetworkAddress, Error> {
        match crate::gio::network_address::from_uri(&self.uri, DEFAULT_PORT) {
            Ok(network_address) => Ok(network_address),
            Err(reason) => Err(Error::NetworkAddress(reason)),
        }
    }
}
