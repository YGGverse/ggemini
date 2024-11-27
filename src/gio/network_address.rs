pub mod error;
pub use error::Error;

use gio::NetworkAddress;
use glib::Uri;

/// Create new valid [NetworkAddress](https://docs.gtk.org/gio/class.NetworkAddress.html) from [Uri](https://docs.gtk.org/glib/struct.Uri.html)
///
/// Useful as:
/// * shared [SocketConnectable](https://docs.gtk.org/gio/iface.SocketConnectable.html) interface
/// * [SNI](https://geminiprotocol.net/docs/protocol-specification.gmi#server-name-indication) record for TLS connections
pub fn from_uri(uri: &Uri, default_port: u16) -> Result<NetworkAddress, Error> {
    Ok(NetworkAddress::new(
        &match uri.host() {
            Some(host) => host,
            None => return Err(Error::Host(uri.to_string())),
        },
        if uri.port().is_positive() {
            uri.port() as u16
        } else {
            default_port
        },
    ))
}
