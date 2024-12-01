pub mod error;
pub mod response;

pub use error::Error;
pub use response::Response;

use gio::{
    prelude::{IOStreamExt, OutputStreamExt, TlsConnectionExt},
    Cancellable, IOStream, NetworkAddress, SocketConnection, TlsCertificate, TlsClientConnection,
};
use glib::{
    object::{Cast, IsA, ObjectExt},
    Bytes, Priority,
};

pub struct Connection {
    pub socket_connection: SocketConnection,
    pub tls_client_connection: TlsClientConnection,
}

impl Connection {
    // Constructors

    /// Create new `Self`
    pub fn new(
        socket_connection: SocketConnection,
        certificate: Option<TlsCertificate>,
        server_identity: Option<NetworkAddress>,
    ) -> Result<Self, Error> {
        Ok(Self {
            tls_client_connection: match TlsClientConnection::new(
                &socket_connection,
                server_identity.as_ref(),
            ) {
                Ok(tls_client_connection) => {
                    // Prevent session resumption (on certificate change in runtime)
                    tls_client_connection.set_property("session-resumption-enabled", false);

                    // Is user session
                    // https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates
                    if let Some(ref certificate) = certificate {
                        tls_client_connection.set_certificate(certificate);
                    }

                    // @TODO handle
                    // https://geminiprotocol.net/docs/protocol-specification.gmi#closing-connections
                    tls_client_connection.set_require_close_notify(true);

                    // @TODO validate
                    // https://geminiprotocol.net/docs/protocol-specification.gmi#tls-server-certificate-validation
                    tls_client_connection.connect_accept_certificate(|_, _, _| true);
                    tls_client_connection
                }
                Err(e) => return Err(Error::TlsClientConnection(e)),
            },
            socket_connection,
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
        self.tls_client_connection
            .output_stream()
            .write_bytes_async(
                &Bytes::from(format!("{query}\r\n").as_bytes()),
                priority,
                Some(&cancellable.clone()),
                move |result| match result {
                    Ok(_) => Response::from_connection_async(
                        self,
                        priority,
                        cancellable,
                        move |result| {
                            callback(match result {
                                Ok(response) => Ok(response),
                                Err(e) => Err(Error::Response(e)),
                            })
                        },
                    ),
                    Err(e) => callback(Err(Error::Stream(e))),
                },
            );
    }

    // Getters

    /// Get [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
    /// for [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html)
    /// or [TlsClientConnection](https://docs.gtk.org/gio/iface.TlsClientConnection.html) (if available)
    /// * compatible with user (certificate) and guest (certificate-less) connection type
    /// * useful also to keep `Connection` active in async I/O context
    pub fn stream(&self) -> impl IsA<IOStream> {
        // * do not replace with `tls_client_connection.base_io_stream()`
        //   as it will not work properly for user certificate sessions!
        match self.tls_client_connection.certificate().is_some() {
            true => self.tls_client_connection.clone().upcast::<IOStream>(), // is user session
            false => self.socket_connection.clone().upcast::<IOStream>(),    // is guest session
        }
    }
}
