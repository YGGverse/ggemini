//! High-level client API to interact with Gemini Socket Server:
//! * https://geminiprotocol.net/docs/protocol-specification.gmi

pub mod connection;
pub mod error;

pub use connection::Connection;
pub use error::Error;

use gio::{
    prelude::{SocketClientExt, TlsConnectionExt},
    Cancellable, SocketClient, SocketClientEvent, SocketProtocol, TlsCertificate,
    TlsClientConnection,
};
use glib::{object::Cast, Priority, Uri};

pub const DEFAULT_TIMEOUT: u32 = 10;

/// Main point where connect external crate
///
/// Provides high-level API for session-safe interaction with
/// [Gemini](https://geminiprotocol.net) socket server
pub struct Client {
    pub socket: SocketClient,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        // Init new socket
        let socket = SocketClient::new();

        // Setup initial configuration for Gemini Protocol
        socket.set_protocol(SocketProtocol::Tcp);
        socket.set_timeout(DEFAULT_TIMEOUT);

        // Connect events
        socket.connect_event(|_, event, _, stream| {
            // Condition applicable only for guest TLS connections
            // * for user certificates validation, see `new_tls_client_connection`
            if event == SocketClientEvent::TlsHandshaking {
                // Begin guest certificate validation
                stream
                    .unwrap()
                    .dynamic_cast_ref::<TlsClientConnection>()
                    .unwrap()
                    .connect_accept_certificate(|_, _, _| true); // @TODO
            }
        });

        // Done
        Self { socket }
    }

    // Actions

    /// High-level method make new async request to given [Uri](https://docs.gtk.org/glib/struct.Uri.html),
    /// callback with new `Response`on success or `Error` on failure
    /// * compatible with user (certificate) and guest (certificate-less) connection types
    /// * disables default `session-resumption-enabled` property to apply certificate change ability in runtime
    pub fn request_async(
        &self,
        uri: Uri,
        priority: Priority,
        cancellable: Cancellable,
        certificate: Option<TlsCertificate>,
        callback: impl Fn(Result<connection::Response, Error>) + 'static,
    ) {
        // Begin new connection
        // * [NetworkAddress](https://docs.gtk.org/gio/class.NetworkAddress.html) required for valid
        //   [SNI](https://geminiprotocol.net/docs/protocol-specification.gmi#server-name-indication)
        match crate::gio::network_address::from_uri(&uri, crate::DEFAULT_PORT) {
            Ok(network_address) => self.socket.connect_async(
                &network_address.clone(),
                Some(&cancellable.clone()),
                move |result| match result {
                    Ok(socket_connection) => {
                        // Wrap required connection dependencies into the struct holder
                        match Connection::new(socket_connection, certificate, Some(network_address))
                        {
                            Ok(connection) => {
                                // Begin new request
                                connection.request_async(
                                    uri.to_string(),
                                    priority,
                                    cancellable,
                                    move |result| match result {
                                        Ok(response) => callback(Ok(response)),
                                        Err(e) => callback(Err(Error::Connection(e))),
                                    },
                                )
                            }
                            Err(e) => callback(Err(Error::Connection(e))),
                        }
                    }
                    Err(e) => callback(Err(Error::Connect(e))),
                },
            ),
            Err(e) => callback(Err(Error::NetworkAddress(e))),
        }
    }
}
