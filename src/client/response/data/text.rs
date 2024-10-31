//! Tools for Text-based response

pub mod error;
pub use error::Error;

// Local dependencies
use gio::{
    prelude::{IOStreamExt, InputStreamExt},
    Cancellable, SocketConnection,
};
use glib::{GString, Priority};

// Default limits
pub const BUFFER_CAPACITY: usize = 0x400; // 1024
pub const BUFFER_MAX_SIZE: usize = 0xfffff; // 1M

/// Container for text-based response data
pub struct Text {
    data: GString,
}

impl Text {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            data: GString::new(),
        }
    }

    /// Create new `Self` from string
    pub fn from_string(data: &str) -> Self {
        Self { data: data.into() }
    }

    /// Create new `Self` from UTF-8 buffer
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, (Error, Option<&str>)> {
        match GString::from_utf8(buffer.into()) {
            Ok(data) => Ok(Self::from_string(&data)),
            Err(_) => Err((Error::Decode, None)),
        }
    }

    /// Asynchronously create new `Self` from [InputStream](https://docs.gtk.org/gio/class.InputStream.html)
    /// for given [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html)
    pub fn from_socket_connection_async(
        socket_connection: SocketConnection,
        priority: Option<Priority>,
        cancellable: Option<Cancellable>,
        on_complete: impl FnOnce(Result<Self, (Error, Option<&str>)>) + 'static,
    ) {
        read_all_from_socket_connection_async(
            Vec::with_capacity(BUFFER_CAPACITY),
            socket_connection,
            match cancellable {
                Some(value) => Some(value),
                None => None::<Cancellable>,
            },
            match priority {
                Some(value) => value,
                None => Priority::DEFAULT,
            },
            |result| match result {
                Ok(buffer) => on_complete(Self::from_utf8(&buffer)),
                Err(reason) => on_complete(Err(reason)),
            },
        );
    }

    // Getters

    /// Get reference to `Self` data
    pub fn data(&self) -> &GString {
        &self.data
    }
}

// Tools

/// Asynchronously read all bytes from [InputStream](https://docs.gtk.org/gio/class.InputStream.html)
/// for given [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html)
///
/// Return UTF-8 buffer collected.
///
/// * this function implements low-level helper for `Text::from_socket_connection_async`, also provides public API for external integrations
/// * requires `SocketConnection` instead of `InputStream` to keep connection alive (by increasing reference count in async context) @TODO
pub fn read_all_from_socket_connection_async(
    mut buffer: Vec<u8>,
    socket_connection: SocketConnection,
    cancelable: Option<Cancellable>,
    priority: Priority,
    callback: impl FnOnce(Result<Vec<u8>, (Error, Option<&str>)>) + 'static,
) {
    socket_connection.input_stream().read_bytes_async(
        BUFFER_CAPACITY,
        priority,
        cancelable.clone().as_ref(),
        move |result| match result {
            Ok(bytes) => {
                // No bytes were read, end of stream
                if bytes.len() == 0 {
                    return callback(Ok(buffer));
                }

                // Validate overflow
                if buffer.len() + bytes.len() > BUFFER_MAX_SIZE {
                    return callback(Err((Error::BufferOverflow, None)));
                }

                // Save chunks to buffer
                for &byte in bytes.iter() {
                    buffer.push(byte);
                }

                // Continue bytes reading
                read_all_from_socket_connection_async(
                    buffer,
                    socket_connection,
                    cancelable,
                    priority,
                    callback,
                );
            }
            Err(reason) => callback(Err((Error::InputStream, Some(reason.message())))),
        },
    );
}
