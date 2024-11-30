pub mod error;
pub use error::Error;

use gio::{
    prelude::{CancellableExt, IOStreamExt, TlsConnectionExt},
    Cancellable, IOStream, NetworkAddress, SocketConnection, TlsCertificate, TlsClientConnection,
};
use glib::object::{Cast, IsA};

pub struct Connection {
    pub cancellable: Option<Cancellable>,
    pub server_identity: Option<NetworkAddress>,
    pub socket_connection: SocketConnection,
    pub tls_client_connection: Option<TlsClientConnection>,
}

impl Connection {
    // Constructors

    /// Create new `Self`
    pub fn new_wrap(
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
            server_identity: server_identity.clone(),
            socket_connection: socket_connection.clone(),
            tls_client_connection: match certificate {
                Some(certificate) => {
                    match new_tls_client_connection(
                        &socket_connection,
                        &certificate,
                        server_identity.as_ref(),
                    ) {
                        Ok(tls_client_connection) => Some(tls_client_connection),
                        Err(e) => return Err(e),
                    }
                }
                None => None,
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

    /// Request force handshake for `Self`
    /// * useful for certificate change in runtime
    /// * support guest and user sessions
    pub fn rehandshake(&self) -> Result<(), Error> {
        match self
            .tls_client_connection()?
            .handshake(self.cancellable.as_ref())
        {
            Ok(()) => Ok(()),
            Err(e) => Err(Error::Rehandshake(e)),
        }
    }

    // Getters

    /// Upcast [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
    /// for [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html)
    /// or [TlsClientConnection](https://docs.gtk.org/gio/iface.TlsClientConnection.html) (if available)
    /// * wanted to keep `Connection` active in async I/O context
    pub fn stream(&self) -> impl IsA<IOStream> {
        match self.tls_client_connection.clone() {
            Some(tls_client_connection) => tls_client_connection.upcast::<IOStream>(),
            None => self.socket_connection.clone().upcast::<IOStream>(),
        }
    }

    /// Get [TlsClientConnection](https://docs.gtk.org/gio/iface.TlsClientConnection.html) for `Self`
    /// * compatible with both user and guest connection types
    pub fn tls_client_connection(&self) -> Result<TlsClientConnection, Error> {
        match self.tls_client_connection.clone() {
            // User session
            Some(tls_client_connection) => Ok(tls_client_connection),
            // Guest session
            None => {
                // Create new wrapper for `IOStream` to interact `TlsClientConnection` API
                match TlsClientConnection::new(
                    self.stream().as_ref(),
                    self.server_identity.as_ref(),
                ) {
                    Ok(tls_client_connection) => Ok(tls_client_connection),
                    Err(e) => Err(Error::TlsClientConnection(e)),
                }
            }
        }
    }
}

// Tools

pub fn new_tls_client_connection(
    socket_connection: &SocketConnection,
    certificate: &TlsCertificate,
    server_identity: Option<&NetworkAddress>,
) -> Result<TlsClientConnection, Error> {
    if socket_connection.is_closed() {
        return Err(Error::Closed);
    }

    // https://geminiprotocol.net/docs/protocol-specification.gmi#the-use-of-tls
    match TlsClientConnection::new(socket_connection, server_identity) {
        Ok(tls_client_connection) => {
            // https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates
            tls_client_connection.set_certificate(certificate);

            // @TODO handle exceptions
            // https://geminiprotocol.net/docs/protocol-specification.gmi#closing-connections
            tls_client_connection.set_require_close_notify(true);

            // @TODO host validation
            // https://geminiprotocol.net/docs/protocol-specification.gmi#tls-server-certificate-validation
            tls_client_connection.connect_accept_certificate(move |_, _, _| true);

            Ok(tls_client_connection)
        }
        Err(e) => Err(Error::TlsClientConnection(e)),
    }
}
