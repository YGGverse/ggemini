//! High-level client API to interact with Gemini Socket Server:
//! * https://geminiprotocol.net/docs/protocol-specification.gmi

pub mod connection;
pub mod error;
pub mod response;

pub use connection::Connection;
pub use error::Error;
pub use response::Response;

use gio::{
    prelude::{IOStreamExt, OutputStreamExt, SocketClientExt, TlsConnectionExt},
    Cancellable, SocketClient, SocketClientEvent, SocketProtocol, TlsCertificate,
    TlsClientConnection,
};
use glib::{object::Cast, Bytes, Priority, Uri};
use std::rc::Rc;

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
        socket.connect_event(move |_, event, _, stream| {
            // This condition have effect only for guest TLS connections
            // * for user certificates validation, use `Connection` impl
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
    pub fn request_async(
        &self,
        uri: Uri,
        priority: Option<Priority>,
        cancellable: Option<Cancellable>,
        certificate: Option<TlsCertificate>,
        callback: impl Fn(Result<Response, Error>) + 'static,
    ) {
        // Toggle socket mode
        // * guest sessions will not work without!
        self.socket.set_tls(certificate.is_none());

        // Begin new connection
        // * [NetworkAddress](https://docs.gtk.org/gio/class.NetworkAddress.html) required for valid
        //   [SNI](https://geminiprotocol.net/docs/protocol-specification.gmi#server-name-indication)
        match crate::gio::network_address::from_uri(&uri, crate::DEFAULT_PORT) {
            Ok(network_address) => self.socket.connect_async(
                &network_address.clone(),
                match cancellable {
                    Some(ref cancellable) => Some(cancellable.clone()),
                    None => None::<Cancellable>,
                }
                .as_ref(),
                move |result| match result {
                    Ok(connection) => {
                        // Wrap required connection dependencies into the struct holder
                        match Connection::new(
                            connection,
                            certificate,
                            Some(network_address),
                            cancellable.clone(),
                        ) {
                            Ok(connection) => {
                                // Begin new request
                                request_async(
                                    Rc::new(connection),
                                    uri.to_string(),
                                    priority,
                                    cancellable,
                                    callback, // result
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

/// Middle-level method to make new request to `Connection`
/// * callback with new `Response`on success or `Error` on failure
pub fn request_async(
    connection: Rc<Connection>,
    query: String,
    priority: Option<Priority>,
    cancellable: Option<Cancellable>,
    callback: impl Fn(Result<Response, Error>) + 'static,
) {
    connection.stream().output_stream().write_bytes_async(
        &Bytes::from(format!("{query}\r\n").as_bytes()),
        match priority {
            Some(priority) => priority,
            None => Priority::DEFAULT,
        },
        match cancellable {
            Some(ref cancellable) => Some(cancellable.clone()),
            None => None::<Cancellable>,
        }
        .as_ref(),
        move |result| match result {
            Ok(_) => {
                Response::from_request_async(connection, priority, cancellable, move |result| {
                    callback(match result {
                        Ok(response) => Ok(response),
                        Err(e) => Err(Error::Response(e)),
                    })
                })
            }
            Err(e) => callback(Err(Error::OutputStream(e))),
        },
    );
}
