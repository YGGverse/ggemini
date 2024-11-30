mod error;
pub use error::Error;

use super::Connection;
use gio::{
    prelude::{TlsCertificateExt, TlsConnectionExt},
    TlsCertificate,
};
use glib::{Uri, UriHideFlags};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// Request sessions holder
/// * useful to keep connections open in async context and / or validate TLS certificate updates in runtime
pub struct Session {
    index: RefCell<HashMap<String, Rc<Connection>>>,
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

impl Session {
    pub fn new() -> Self {
        Self {
            index: RefCell::new(HashMap::new()),
        }
    }

    pub fn set(&self, request: String, connection: Rc<Connection>) -> Option<Rc<Connection>> {
        self.index.borrow_mut().insert(request, connection)
    }

    /// Update existing session match [scope](https://geminiprotocol.net/docs/protocol-specification.gmi#status-60)
    /// for given [Uri](https://docs.gtk.org/glib/struct.Uri.html)
    /// and [TlsCertificate](https://docs.gtk.org/gio/class.TlsCertificate.html)
    ///
    /// * force rehandshake on user certificate change in runtime (ignore default session resumption by Glib TLS backend)
    pub fn update(&self, uri: &Uri, certificate: Option<&TlsCertificate>) -> Result<(), Error> {
        // Get cached `Client` connections match `uri` scope
        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-60
        for (request, connection) in self.index.borrow().iter() {
            if request.starts_with(
                uri.to_string_partial(UriHideFlags::QUERY | UriHideFlags::FRAGMENT)
                    .as_str(),
            ) {
                // Begin re-handshake on `certificate` change
                match connection.tls_client_connection {
                    // User certificate session
                    Some(ref tls_client_connection) => {
                        match certificate {
                            Some(new) => {
                                // Get previous certificate
                                if let Some(ref old) = tls_client_connection.certificate() {
                                    // User -> User
                                    if !new.is_same(old) {
                                        rehandshake(connection.as_ref());
                                    }
                                }
                            }
                            // User -> Guest
                            None => rehandshake(connection.as_ref()),
                        }
                    }
                    // Guest
                    None => {
                        // Guest -> User
                        if certificate.is_some() {
                            rehandshake(connection.as_ref())
                        }
                    }
                }
            }
        }
        Ok(()) // @TODO result does nothing yet
    }
}

// Tools

/// Applies re-handshake to `Connection`
/// to prevent default session resumption on user certificate change in runtime
pub fn rehandshake(connection: &Connection) {
    if let Err(e) = connection.rehandshake() {
        println!("warning: {e}"); // @TODO keep in mind until solution for TLS 1.3
    }
}
