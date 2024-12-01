//! High-level client API to interact with Gemini Socket Server:
//! * https://geminiprotocol.net/docs/protocol-specification.gmi

pub mod connection;
pub mod error;

pub use connection::Connection;
pub use error::Error;

use gio::{prelude::SocketClientExt, Cancellable, SocketClient, SocketProtocol, TlsCertificate};
use glib::{Priority, Uri};

// Defaults

pub const DEFAULT_TIMEOUT: u32 = 10;
pub const DEFAULT_SESSION_RESUMPTION: bool = false;

/// Main point where connect external crate
///
/// Provides high-level API for session-safe interaction with
/// [Gemini](https://geminiprotocol.net) socket server
pub struct Client {
    is_session_resumption: bool,
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

        // Done
        Self {
            is_session_resumption: DEFAULT_SESSION_RESUMPTION,
            socket,
        }
    }

    // Actions

    /// Make new async request to given [Uri](https://docs.gtk.org/glib/struct.Uri.html),
    /// callback with new `Response`on success or `Error` on failure
    /// * compatible with user (certificate) and guest (certificate-less) connection types
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
            Ok(network_address) => {
                self.socket
                    .connect_async(&network_address.clone(), Some(&cancellable.clone()), {
                        let is_session_resumption = self.is_session_resumption;
                        move |result| match result {
                            Ok(socket_connection) => {
                                match Connection::new(
                                    socket_connection,
                                    certificate,
                                    Some(network_address),
                                    is_session_resumption,
                                ) {
                                    Ok(connection) => connection.request_async(
                                        uri.to_string(),
                                        priority,
                                        cancellable,
                                        move |result| match result {
                                            Ok(response) => callback(Ok(response)),
                                            Err(e) => callback(Err(Error::Connection(e))),
                                        },
                                    ),
                                    Err(e) => callback(Err(Error::Connection(e))),
                                }
                            }
                            Err(e) => callback(Err(Error::Connect(e))),
                        }
                    })
            }
            Err(e) => callback(Err(Error::NetworkAddress(e))),
        }
    }

    // Setters

    /// Change glib-networking `session-resumption-enabled` property (`false` by default)
    /// * [Gemini specification](https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates)
    /// * [GnuTLS manual](https://www.gnutls.org/manual/html_node/Session-resumption.html)
    pub fn set_session_resumption(&mut self, is_enabled: bool) {
        self.is_session_resumption = is_enabled
    }
}
