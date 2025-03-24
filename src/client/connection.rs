pub mod error;
pub mod request;
pub mod response;

pub use error::Error;
pub use request::{Mode, Request};
pub use response::Response;

// Local dependencies

use gio::{
    Cancellable, IOStream, NetworkAddress, SocketConnection, TlsCertificate, TlsClientConnection,
    prelude::{IOStreamExt, OutputStreamExtManual, TlsConnectionExt},
};
use glib::{
    Bytes, Priority,
    object::{Cast, ObjectExt},
};

pub struct Connection {
    pub network_address: NetworkAddress,
    pub socket_connection: SocketConnection,
    pub tls_client_connection: TlsClientConnection,
}

impl Connection {
    // Constructors

    /// Create new `Self`
    pub fn build(
        socket_connection: SocketConnection,
        network_address: NetworkAddress,
        certificate: Option<TlsCertificate>,
        is_session_resumption: bool,
    ) -> Result<Self, Error> {
        Ok(Self {
            tls_client_connection: match new_tls_client_connection(
                &socket_connection,
                Some(&network_address),
                is_session_resumption,
            ) {
                Ok(tls_client_connection) => {
                    if let Some(ref c) = certificate {
                        tls_client_connection.set_certificate(c);
                    }
                    tls_client_connection
                }
                Err(e) => return Err(e),
            },
            network_address,
            socket_connection,
        })
    }

    // Actions

    /// Send new `Request` to `Self` connection using
    /// [Gemini](https://geminiprotocol.net/docs/protocol-specification.gmi) or
    /// [Titan](gemini://transjovian.org/titan/page/The%20Titan%20Specification) protocol
    pub fn request_async(
        self,
        request: Request,
        priority: Priority,
        cancellable: Cancellable,
        callback: impl FnOnce(Result<(Response, Self), Error>) + 'static,
    ) {
        let output_stream = self.stream().output_stream();
        // Make sure **all header bytes** sent to the destination
        // > A partial write is performed with the size of a message block, which is 16kB
        // > https://docs.openssl.org/3.0/man3/SSL_write/#notes
        output_stream.clone().write_all_async(
            Bytes::from_owned(request.header()),
            priority,
            Some(&cancellable.clone()),
            move |result| match result {
                Ok(_) => match request {
                    Request::Gemini { mode, .. } => match mode {
                        Mode::All => todo!(),
                        Mode::Header => Response::header_from_connection_async(
                            self,
                            priority,
                            cancellable,
                            |result, connection| {
                                callback(match result {
                                    Ok(response) => Ok((response, connection)),
                                    Err(e) => Err(Error::Response(e)),
                                })
                            },
                        ),
                    },
                    // Make sure **all data bytes** sent to the destination
                    // > A partial write is performed with the size of a message block, which is 16kB
                    // > https://docs.openssl.org/3.0/man3/SSL_write/#notes
                    Request::Titan { data, mode, .. } => output_stream.write_all_async(
                        data,
                        priority,
                        Some(&cancellable.clone()),
                        move |result| match result {
                            Ok(_) => match mode {
                                Mode::All => todo!(),
                                Mode::Header => Response::header_from_connection_async(
                                    self,
                                    priority,
                                    cancellable,
                                    |result, connection| {
                                        callback(match result {
                                            Ok(response) => Ok((response, connection)),
                                            Err(e) => Err(Error::Response(e)),
                                        })
                                    },
                                ),
                            },
                            Err((b, e)) => callback(Err(Error::Request(b, e))),
                        },
                    ),
                },
                Err((b, e)) => callback(Err(Error::Request(b, e))),
            },
        )
    }

    // Getters

    /// Get [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
    /// * compatible with user (certificate) and guest (certificate-less) connection type
    /// * useful to keep `Connection` reference active in async I/O context
    pub fn stream(&self) -> IOStream {
        self.tls_client_connection.clone().upcast::<IOStream>()
        // * also `base_io_stream` method available @TODO
    }
}

// Tools

/// Setup new [TlsClientConnection](https://docs.gtk.org/gio/iface.TlsClientConnection.html)
/// wrapper for [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html)
/// using `server_identity` as the [SNI](https://geminiprotocol.net/docs/protocol-specification.gmi#server-name-indication)
fn new_tls_client_connection(
    socket_connection: &SocketConnection,
    server_identity: Option<&NetworkAddress>,
    is_session_resumption: bool,
) -> Result<TlsClientConnection, Error> {
    match TlsClientConnection::new(socket_connection, server_identity) {
        Ok(tls_client_connection) => {
            // Prevent session resumption (certificate change ability in runtime)
            tls_client_connection.set_property("session-resumption-enabled", is_session_resumption);

            // Return `Err` on server connection mismatch following specification lines:
            // > Gemini servers MUST use the TLS close_notify implementation to close the connection
            // > A client SHOULD notify the user of such a case
            // https://geminiprotocol.net/docs/protocol-specification.gmi#closing-connections
            tls_client_connection.set_require_close_notify(true);

            // @TODO validate
            // https://geminiprotocol.net/docs/protocol-specification.gmi#tls-server-certificate-validation
            tls_client_connection.connect_accept_certificate(|_, _, _| true);

            Ok(tls_client_connection)
        }
        Err(e) => Err(Error::TlsClientConnection(e)),
    }
}
