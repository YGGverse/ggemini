pub mod error;
pub use error::Error;

use gio::{
    prelude::{IOStreamExt, TlsConnectionExt},
    IOStream, NetworkAddress, SocketConnection, TlsCertificate, TlsClientConnection,
};
use glib::object::{Cast, IsA};

pub struct Connection {
    pub socket_connection: SocketConnection,
    pub tls_client_connection: Option<TlsClientConnection>,
}

impl Connection {
    // Constructors

    /// Create new `Self`
    pub fn new_for(
        socket_connection: &SocketConnection,
        certificate: Option<&TlsCertificate>,
        server_identity: Option<&NetworkAddress>,
    ) -> Result<Self, Error> {
        if socket_connection.is_closed() {
            return Err(Error::SocketConnectionClosed);
        }

        Ok(Self {
            socket_connection: socket_connection.clone(),
            tls_client_connection: match certificate {
                Some(certificate) => match auth(socket_connection, certificate, server_identity) {
                    Ok(tls_client_connection) => Some(tls_client_connection),
                    Err(reason) => return Err(reason),
                },
                None => None,
            },
        })
    }

    // Getters

    pub fn stream(&self) -> impl IsA<IOStream> {
        match self.tls_client_connection.clone() {
            Some(tls_client_connection) => tls_client_connection.upcast::<IOStream>(),
            None => self.socket_connection.clone().upcast::<IOStream>(),
        }
    }
}

// Tools

pub fn auth(
    socket_connection: &SocketConnection,
    certificate: &TlsCertificate,
    server_identity: Option<&NetworkAddress>,
) -> Result<TlsClientConnection, Error> {
    if socket_connection.is_closed() {
        return Err(Error::SocketConnectionClosed);
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
        Err(reason) => Err(Error::TlsClientConnection(reason)),
    }
}
