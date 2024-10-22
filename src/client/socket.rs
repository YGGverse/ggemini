use gio::{prelude::SocketClientExt, SocketClient, SocketProtocol, TlsCertificateFlags};

pub struct Socket {
    gobject: SocketClient,
}

impl Socket {
    /// Create new `gio::SocketClient` preset for Gemini Protocol
    pub fn new() -> Self {
        let gobject = SocketClient::new();

        gobject.set_protocol(SocketProtocol::Tcp);
        gobject.set_tls_validation_flags(TlsCertificateFlags::INSECURE);
        gobject.set_tls(true);

        Self { gobject }
    }

    /// Return ref to `gio::SocketClient` GObject
    pub fn gobject(&self) -> &SocketClient {
        self.gobject.as_ref()
    }
}
