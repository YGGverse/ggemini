//! High-level client API to interact with Gemini Socket Server:
//! * https://geminiprotocol.net/docs/protocol-specification.gmi

pub mod certificate;
pub mod connection;
pub mod error;
pub mod response;

pub use certificate::Certificate;
pub use connection::Connection;
pub use error::Error;
pub use response::Response;

use gio::{
    prelude::{IOStreamExt, OutputStreamExt, SocketClientExt},
    Cancellable, SocketClient, SocketProtocol, TlsCertificate,
};
use glib::{Bytes, Priority, Uri};

pub const DEFAULT_TIMEOUT: u32 = 10;

pub struct Client {
    pub socket: SocketClient,
}

impl Client {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        let socket = SocketClient::new();

        socket.set_protocol(SocketProtocol::Tcp);
        socket.set_timeout(DEFAULT_TIMEOUT);

        Self { socket }
    }

    // Actions

    /// Make async request to given [Uri](https://docs.gtk.org/glib/struct.Uri.html),
    /// callback with `Response`on success or `Error` on failure.
    /// * creates new [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html)
    /// * session management by Glib TLS Backend
    pub fn request_async(
        &self,
        uri: Uri,
        priority: Option<Priority>,
        cancellable: Option<Cancellable>,
        certificate: Option<TlsCertificate>,
        callback: impl Fn(Result<Response, Error>) + 'static,
    ) {
        match crate::gio::network_address::from_uri(&uri, crate::DEFAULT_PORT) {
            Ok(network_address) => {
                self.socket.connect_async(
                    &network_address.clone(),
                    match cancellable {
                        Some(ref cancellable) => Some(cancellable.clone()),
                        None => None::<Cancellable>,
                    }
                    .as_ref(),
                    move |result| match result {
                        Ok(connection) => {
                            match Connection::new_for(
                                &connection,
                                certificate.as_ref(),
                                Some(&network_address),
                            ) {
                                Ok(result) => request_async(
                                    result,
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
                                ),
                                Err(reason) => callback(Err(Error::Connection(reason))),
                            }
                        }
                        Err(reason) => callback(Err(Error::Connect(reason))),
                    },
                );
            }
            Err(reason) => callback(Err(Error::NetworkAddress(reason))),
        };
    }
}

/// Make new request for constructed `Connection`
/// * callback with `Response`on success or `Error` on failure
pub fn request_async(
    connection: Connection,
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
