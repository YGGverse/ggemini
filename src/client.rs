//! High-level client API to interact with Gemini Socket Server:
//! * https://geminiprotocol.net/docs/protocol-specification.gmi

pub mod connection;
pub mod error;
pub mod response;
pub mod session;

pub use connection::Connection;
pub use error::Error;
pub use response::Response;
pub use session::Session;

use gio::{
    prelude::{IOStreamExt, OutputStreamExt, SocketClientExt, TlsConnectionExt},
    Cancellable, SocketClient, SocketClientEvent, SocketProtocol, TlsCertificate,
    TlsClientConnection,
};
use glib::{object::Cast, Bytes, Priority, Uri};
use std::rc::Rc;

pub const DEFAULT_TIMEOUT: u32 = 10;

pub struct Client {
    session: Rc<Session>,
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
        Self {
            session: Rc::new(Session::new()),
            socket,
        }
    }

    // Actions

    /// Make new async request to given [Uri](https://docs.gtk.org/glib/struct.Uri.html),
    /// callback with new `Response`on success or `Error` on failure
    ///
    /// * method does not close new `Connection` created, hold it in `Session`,
    ///   expects from user manual `Response` handle with close act on complete
    ///   * if new request match same `uri`, method auto-close previous connection, renew `Session`
    /// * method ignores default session resumption provided by Glib TLS backend,
    ///   implement certificate change ability in application runtime
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

        // Update previous session if available for this `uri`, force rehandshake on certificate change
        match self.session.update(&uri, certificate.as_ref()) {
            // Begin new connection
            // * [NetworkAddress](https://docs.gtk.org/gio/class.NetworkAddress.html) required for valid
            //   [SNI](https://geminiprotocol.net/docs/protocol-specification.gmi#server-name-indication)
            Ok(()) => match crate::gio::network_address::from_uri(&uri, crate::DEFAULT_PORT) {
                Ok(network_address) => self.socket.connect_async(
                    &network_address.clone(),
                    match cancellable {
                        Some(ref cancellable) => Some(cancellable.clone()),
                        None => None::<Cancellable>,
                    }
                    .as_ref(),
                    {
                        let session = self.session.clone();
                        move |result| match result {
                            Ok(connection) => {
                                // Wrap required connection dependencies into the struct holder
                                match Connection::new_wrap(
                                    connection,
                                    certificate,
                                    Some(network_address),
                                ) {
                                    Ok(connection) => {
                                        // Wrap to shared reference support clone semantics
                                        let connection = Rc::new(connection);

                                        // Renew session
                                        session.set(uri.to_string(), connection.clone());

                                        // Begin new request
                                        request_async(
                                            connection,
                                            uri.to_string(),
                                            match priority {
                                                Some(priority) => Some(priority),
                                                None => Some(Priority::DEFAULT),
                                            },
                                            match cancellable {
                                                Some(ref cancellable) => Some(cancellable.clone()),
                                                None => None::<Cancellable>,
                                            },
                                            callback, // callback with response
                                        )
                                    }
                                    Err(e) => callback(Err(Error::Connection(e))),
                                }
                            }
                            Err(e) => callback(Err(Error::Connect(e))),
                        }
                    },
                ),
                Err(e) => callback(Err(Error::NetworkAddress(e))),
            },
            Err(e) => callback(Err(Error::Session(e))),
        }
    }
}

/// Make new request for constructed `Connection`
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
