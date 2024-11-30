mod error;
pub use error::Error;

use super::Connection;
use gio::{
    prelude::{TlsCertificateExt, TlsConnectionExt},
    Cancellable, TlsCertificate,
};
use glib::Uri;
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

    pub fn get(&self, request: &str) -> Option<Rc<Connection>> {
        self.index.borrow().get(request).cloned()
    }

    pub fn set(&self, request: String, connection: Rc<Connection>) -> Option<Rc<Connection>> {
        self.index.borrow_mut().insert(request, connection)
    }

    /// Update existing session for given [Uri](https://docs.gtk.org/glib/struct.Uri.html)
    /// and [TlsCertificate](https://docs.gtk.org/gio/class.TlsCertificate.html)
    /// * force rehandshake on user certificate was changed in runtime (ignore default session resumption by Glib TLS backend implementation)
    /// * close previous connection match `Uri` if not closed yet
    pub fn update(&self, uri: &Uri, certificate: Option<&TlsCertificate>) -> Result<(), Error> {
        if let Some(connection) = self.get(&uri.to_string()) {
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

            // Close connection if active yet
            if let Err(reason) = connection.close(Cancellable::NONE) {
                return Err(Error::Connection(reason));
            }
        }
        Ok(())
    }
}

// Tools

// Applies re-handshake to `Connection` to prevent default session resumption
// on user certificate change in runtime
pub fn rehandshake(connection: &Connection) {
    if let Err(e) = connection.rehandshake() {
        println!("warning: {e}"); // @TODO keep in mind until solution for TLS 1.3
    }
}
