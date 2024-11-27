//! High-level client API to interact with Gemini Socket Server:
//! * https://geminiprotocol.net/docs/protocol-specification.gmi

pub mod connection;
pub mod error;
pub mod response;

pub use connection::Connection;
pub use error::Error;
pub use response::Response;

use gio::{
    prelude::{IOStreamExt, OutputStreamExt, SocketClientExt},
    Cancellable, NetworkAddress, SocketClient, SocketProtocol, TlsCertificate,
};
use glib::{Bytes, Priority, Uri};

pub const DEFAULT_PORT: u16 = 1965;
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
    /// callback with `Result`on success or `Error` on failure.
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
        match network_address_for(&uri) {
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
                            match Connection::from(network_address, connection, certificate) {
                                Ok(result) => request_async(
                                    result,
                                    uri.to_string(),
                                    match priority {
                                        Some(priority) => priority,
                                        None => Priority::DEFAULT,
                                    },
                                    cancellable.unwrap(), // @TODO
                                    move |result| callback(result),
                                ),
                                Err(reason) => callback(Err(Error::Connection(reason))),
                            }
                        }
                        Err(reason) => callback(Err(Error::Connect(reason))),
                    },
                );
            }
            Err(reason) => callback(Err(reason)),
        };
    }
}

// Private helpers

/// [SocketConnectable](https://docs.gtk.org/gio/iface.SocketConnectable.html) /
/// [SNI](https://geminiprotocol.net/docs/protocol-specification.gmi#server-name-indication)
fn network_address_for(uri: &Uri) -> Result<NetworkAddress, Error> {
    Ok(NetworkAddress::new(
        &match uri.host() {
            Some(host) => host,
            None => return Err(Error::Connectable(uri.to_string())),
        },
        if uri.port().is_positive() {
            uri.port() as u16
        } else {
            DEFAULT_PORT
        },
    ))
}

fn request_async(
    connection: Connection,
    query: String,
    priority: Priority,
    cancellable: Cancellable,
    callback: impl Fn(Result<Response, Error>) + 'static,
) {
    connection.stream().output_stream().write_bytes_async(
        &Bytes::from(format!("{query}\r\n").as_bytes()),
        priority,
        Some(&cancellable.clone()),
        move |result| match result {
            Ok(_) => Response::from_request_async(
                connection,
                Some(priority),
                Some(cancellable),
                move |result| match result {
                    Ok(response) => callback(Ok(response)),
                    Err(reason) => callback(Err(Error::Response(reason))),
                },
            ),
            Err(reason) => callback(Err(Error::Write(reason))),
        },
    );
}
