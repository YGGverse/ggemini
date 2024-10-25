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
}

impl Socket {
    // Constructors

    /// Create new `gio::SocketClient` preset for Gemini Protocol
    pub fn new() -> Self {
        let client = SocketClient::new();

        client.set_protocol(SocketProtocol::Tcp);
        client.set_tls_validation_flags(TlsCertificateFlags::INSECURE);
        client.set_tls(true);

        Self { client }
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
            DEFAULT_PORT,
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

    // Getters

    /// Return ref to `gio::SocketClient` GObject
    pub fn client(&self) -> &SocketClient {
        self.client.as_ref()
    }
}
