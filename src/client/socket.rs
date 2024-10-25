pub mod connection;
pub mod error;

pub use connection::Connection;
pub use error::Error;

pub const DEFAULT_PORT: u16 = 1965;

use gio::{
    prelude::SocketClientExt, Cancellable, SocketClient, SocketProtocol, TlsCertificateFlags,
};
use glib::Uri;

pub struct Socket {
    client: SocketClient,
    default_port: u16,
}

impl Socket {
    // Constructors

    /// Create new `gio::SocketClient` preset for Gemini Protocol
    pub fn new() -> Self {
        let client = SocketClient::new();

        client.set_protocol(SocketProtocol::Tcp);
        client.set_tls_validation_flags(TlsCertificateFlags::INSECURE);
        client.set_tls(true);

        Self {
            client,
            default_port: DEFAULT_PORT,
        }
    }

    // Actions
    pub fn connect_async(
        &self,
        uri: Uri,
        cancelable: Option<Cancellable>,
        callback: impl FnOnce(Result<Connection, Error>) + 'static,
    ) {
        self.client.connect_to_uri_async(
            uri.to_str().as_str(),
            self.default_port,
            match cancelable.clone() {
                Some(value) => Some(value),
                None => None::<Cancellable>,
            }
            .as_ref(),
            |result| {
                callback(match result {
                    Ok(connection) => Ok(Connection::new_from(connection)),
                    Err(_) => Err(Error::Connection),
                })
            },
        );
    }

    // Setters

    /// Set default port for socket connections (1965 by default)
    pub fn set_default_port(&mut self, default_port: u16) {
        self.default_port = default_port;
    }

    // Getters

    /// Get reference to `gio::SocketClient`
    ///
    /// https://docs.gtk.org/gio/class.SocketClient.html
    pub fn client(&self) -> &SocketClient {
        self.client.as_ref()
    }
}
