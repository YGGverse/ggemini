pub mod error;
pub use error::Error;

use gio::{
    prelude::{CancellableExt, IOStreamExt, TlsConnectionExt},
    Cancellable, IOStream, NetworkAddress, SocketConnection, TlsCertificate, TlsClientConnection,
};
use glib::object::{Cast, IsA, ObjectExt};

pub struct Connection {
    pub cancellable: Option<Cancellable>,
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
        cancellable: Option<Cancellable>,
    ) -> Result<Self, Error> {
        if socket_connection.is_closed() {
            return Err(Error::Closed);
        }

        Ok(Self {
            cancellable,
            socket_connection: socket_connection.clone(),
            tls_client_connection: match TlsClientConnection::new(
                &socket_connection.clone(),
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
                    tls_client_connection.connect_accept_certificate(move |_, _, _| true);
                    tls_client_connection
                }
                Err(e) => return Err(Error::TlsClientConnection(e)),
            },
        })
    }

    // Actions

    /// Apply `cancel` action to `Self` [Cancellable](https://docs.gtk.org/gio/method.Cancellable.cancel.html)
    /// * return `Error` on `Cancellable` not found
    pub fn cancel(&self) -> Result<(), Error> {
        match self.cancellable {
            Some(ref cancellable) => Ok(cancellable.cancel()),
            None => Err(Error::Cancel),
        }
    }

    /// Close owned [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html)
    /// * return `Ok(false)` if `Cancellable` not defined
    pub fn close(&self) -> Result<bool, Error> {
        match self.cancellable {
            Some(ref cancellable) => match self.socket_connection.close(Some(cancellable)) {
                Ok(()) => Ok(true),
                Err(e) => Err(Error::SocketConnection(e)),
            },
            None => Ok(false),
        }
    }

    // Getters

    /// Get [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
    /// for [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html)
    /// or [TlsClientConnection](https://docs.gtk.org/gio/iface.TlsClientConnection.html) (if available)
    /// * compatible with user (certificate) and guest (certificate-less) connection types
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
