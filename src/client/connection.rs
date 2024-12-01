pub mod error;
pub mod response;

pub use error::Error;
pub use response::Response;

use gio::{
    prelude::{IOStreamExt, OutputStreamExt, TlsConnectionExt},
    Cancellable, IOStream, NetworkAddress, SocketConnection, TlsCertificate, TlsClientConnection,
};
use glib::{
    object::{Cast, ObjectExt},
    Bytes, Priority,
};

pub struct Connection {
    pub tls_client_connection: TlsClientConnection,
}

impl Connection {
    // Constructors

    /// Create new `Self`
    pub fn new(
        socket_connection: SocketConnection,
        certificate: Option<TlsCertificate>,
        server_identity: Option<NetworkAddress>,
        is_session_resumption: bool,
    ) -> Result<Self, Error> {
        Ok(Self {
            tls_client_connection: match new_tls_client_connection(
                &socket_connection,
                server_identity.as_ref(),
                is_session_resumption,
            ) {
                Ok(tls_client_connection) => {
                    if let Some(ref certificate) = certificate {
                        tls_client_connection.set_certificate(certificate);
                    }
                    tls_client_connection
                }
                Err(e) => return Err(e),
            },
        })
    }

    // Actions

    /// Make new request to `Self` connection
    /// * callback with new `Response` on success or `Error` on failure
    pub fn request_async(
        self,
        query: String,
        priority: Priority,
        cancellable: Cancellable,
        callback: impl Fn(Result<Response, Error>) + 'static,
    ) {
        // Send request
        self.stream().output_stream().write_bytes_async(
            &Bytes::from(format!("{query}\r\n").as_bytes()),
            priority,
            Some(&cancellable.clone()),
            move |result| match result {
                Ok(_) => {
                    // Read response
                    Response::from_connection_async(self, priority, cancellable, move |result| {
                        callback(match result {
                            Ok(response) => Ok(response),
                            Err(e) => Err(Error::Response(e)),
                        })
                    })
                }
                Err(e) => callback(Err(Error::Stream(e))),
            },
        );
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

// Helpers

/// Setup new [TlsClientConnection](https://docs.gtk.org/gio/iface.TlsClientConnection.html)
/// wrapper for [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html)
/// using `server_identity` as [SNI](https://geminiprotocol.net/docs/protocol-specification.gmi#server-name-indication)
pub fn new_tls_client_connection(
    socket_connection: &SocketConnection,
    server_identity: Option<&NetworkAddress>,
    is_session_resumption: bool,
) -> Result<TlsClientConnection, Error> {
    match TlsClientConnection::new(socket_connection, server_identity) {
        Ok(tls_client_connection) => {
            // Prevent session resumption (certificate change ability in runtime)
            tls_client_connection.set_property("session-resumption-enabled", is_session_resumption);

            // @TODO handle
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
