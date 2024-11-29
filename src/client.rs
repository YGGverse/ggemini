//! High-level client API to interact with Gemini Socket Server:
//! * https://geminiprotocol.net/docs/protocol-specification.gmi

pub mod connection;
pub mod error;
pub mod response;
pub mod session;

use std::rc::Rc;

pub use connection::Connection;
pub use error::Error;
pub use response::Response;
pub use session::Session;

use gio::{
    prelude::{IOStreamExt, OutputStreamExt, SocketClientExt, TlsCertificateExt, TlsConnectionExt},
    Cancellable, SocketClient, SocketClientEvent, SocketProtocol, TlsCertificate,
    TlsClientConnection,
};
use glib::{object::Cast, Bytes, Priority, Uri};

pub const DEFAULT_TIMEOUT: u32 = 10;

pub struct Client {
    session: Rc<Session>,
    pub socket: SocketClient,
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
    /// callback with new `Response`on success or `Error` on failure.
    /// * call this method ignore default session resumption by Glib TLS backend,
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

        // Update previous session available for this request
        match self.update_session(&uri, certificate.as_ref()) {
            // Begin new connection
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
                                match Connection::new_wrap(
                                    connection,
                                    certificate,
                                    Some(network_address),
                                ) {
                                    Ok(connection) => {
                                        // Wrap connection to shared reference clone semantics
                                        let connection = Rc::new(connection);

                                        // Update session
                                        session.update(uri.to_string(), connection.clone());

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
                                            move |result| callback(result),
                                        )
                                    }
                                    Err(reason) => callback(Err(Error::Connection(reason))),
                                }
                            }
                            Err(reason) => callback(Err(Error::Connect(reason))),
                        }
                    },
                ),
                Err(reason) => callback(Err(Error::NetworkAddress(reason))),
            },
            Err(reason) => callback(Err(reason)),
        }
    }

    /// Update existing session for given request
    pub fn update_session(
        &self,
        uri: &Uri,
        certificate: Option<&TlsCertificate>,
    ) -> Result<(), Error> {
        if let Some(connection) = self.session.get(&uri.to_string()) {
            // Check connection contain TLS authorization
            match connection.tls_client_connection {
                Some(ref tls_client_connection) => {
                    match certificate {
                        Some(new) => {
                            // Get previous certificate
                            if let Some(ref old) = tls_client_connection.certificate() {
                                // User -> User
                                if !new.is_same(old) {
                                    // Prevent session resumption
                                    // Glib backend restore session in runtime with old certificate
                                    // @TODO keep in mind, until better solution found for TLS 1.3
                                    println!("{:?}", connection.rehandshake());
                                }
                            }
                        }
                        None => {
                            // User -> Guest
                            println!("{:?}", connection.rehandshake());
                        }
                    }
                }
                None => {
                    // Guest -> User
                    if certificate.is_some() {
                        println!("{:?}", connection.rehandshake());
                    }
                }
            }

            // Close connection if active yet
            if let Err(reason) = connection.close(Cancellable::NONE) {
                return Err(Error::Connection(reason));
            }
        }
        Ok(())
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
                        Err(reason) => Err(Error::Response(reason)),
                    })
                })
            }
            Err(reason) => callback(Err(Error::OutputStream(reason))),
        },
    );
}
