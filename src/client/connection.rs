pub mod error;
pub mod request;
pub mod response;

pub use error::Error;
pub use request::{Gemini, Request, Titan};
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

    /// Send new `Request` to `Self` connection using
    /// [Gemini](https://geminiprotocol.net/docs/protocol-specification.gmi) or
    /// [Titan](gemini://transjovian.org/titan/page/The%20Titan%20Specification) protocol
    pub fn request_async(
        self,
        request: Request,
        priority: Priority,
        cancellable: Cancellable,
        callback: impl FnOnce(Result<Response, Error>) + 'static,
    ) {
        self.bytes_request_async(&request.to_bytes(), priority, cancellable, callback);
    }

    /// Low-level shared method to send raw bytes array over
    /// [Gemini](https://geminiprotocol.net/docs/protocol-specification.gmi) or
    /// [Titan](gemini://transjovian.org/titan/page/The%20Titan%20Specification) protocol
    /// * bytes array should include formatted header according to protocol selected
    /// * for high-level requests see `gemini_request_async` and `titan_request_async` methods
    /// * to construct multi-protocol request with single function, use `request_async` method
    pub fn bytes_request_async(
        self,
        request: &Bytes,
        priority: Priority,
        cancellable: Cancellable,
        callback: impl FnOnce(Result<Response, Error>) + 'static,
    ) {
        self.stream().output_stream().write_bytes_async(
            request,
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
