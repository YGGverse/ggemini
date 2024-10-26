pub mod error;
pub use error::Error;

use gio::{
    prelude::{IOStreamExt, InputStreamExt},
    Cancellable, SocketConnection,
};
use glib::{Bytes, Priority};

pub const DEFAULT_CAPACITY: usize = 0x400;
pub const DEFAULT_MAX_SIZE: usize = 0xfffff;

/// Dynamically allocated [Bytes](https://docs.gtk.org/glib/struct.Bytes.html) buffer
/// with configurable `capacity` and `max_size` limits
pub struct Buffer {
    buffer: Vec<Bytes>,
    max_size: usize,
}

impl Buffer {
    // Constructors

    /// Create new `Self` with default `capacity` and `max_size` preset
    pub fn new() -> Self {
        Self::new_with_options(Some(DEFAULT_CAPACITY), Some(DEFAULT_MAX_SIZE))
    }

    /// Create new `Self` with options
    ///
    /// Options:
    /// * `capacity` initial bytes request to reduce extra memory reallocation (`DEFAULT_CAPACITY` if `None`)
    /// * `max_size` max bytes to prevent memory overflow by unknown stream source (`DEFAULT_MAX_SIZE` if `None`)
    pub fn new_with_options(capacity: Option<usize>, max_size: Option<usize>) -> Self {
        Self {
            buffer: Vec::with_capacity(match capacity {
                Some(value) => value,
                None => DEFAULT_CAPACITY,
            }),
            max_size: match max_size {
                Some(value) => value,
                None => DEFAULT_MAX_SIZE,
            },
        }
    }

    // Intentable constructors

    /// Simplest way to create `Self` buffer from [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html)
    ///
    /// Options:
    /// * `connection` - [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html) to read bytes from
    /// * `callback` function to apply on all async operations complete, return `Result<Self, (Error, Option<&str>)>`
    pub fn from_connection_async(
        connection: SocketConnection,
        callback: impl FnOnce(Result<Self, (Error, Option<&str>)>) + 'static,
    ) {
        Self::read_all_async(Self::new(), connection, None, None, None, callback);
    }

    // Actions

    /// Asynchronously read all [Bytes](https://docs.gtk.org/glib/struct.Bytes.html)
    /// from [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html) to `Self.buffer`
    ///
    /// Useful to grab entire stream without risk of memory overflow (according to `Self.max_size`),
    /// reduce extra memory reallocations by `capacity` option.
    ///
    /// **Notes**
    ///
    /// We are using entire [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html) reference
    /// instead of [InputStream](https://docs.gtk.org/gio/class.InputStream.html) directly just to keep main connection alive in the async context
    ///
    /// **Options**
    /// * `connection` - [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html) to read bytes from
    /// * `cancellable` - [Cancellable](https://docs.gtk.org/gio/class.Cancellable.html) or `None::<&Cancellable>` by default
    /// * `priority` - [Priority::DEFAULT](https://docs.gtk.org/glib/const.PRIORITY_DEFAULT.html) by default
    /// * `chunk` optional bytes count to read per chunk (`0x100` by default)
    /// * `callback` function to apply on all async operations complete, return `Result<Self, (Error, Option<&str>)>`
    pub fn read_all_async(
        mut self,
        connection: SocketConnection,
        cancelable: Option<Cancellable>,
        priority: Option<Priority>,
        chunk: Option<usize>,
        callback: impl FnOnce(Result<Self, (Error, Option<&str>)>) + 'static,
    ) {
        connection.input_stream().read_bytes_async(
            match chunk {
                Some(value) => value,
                None => 0x100,
            },
            match priority {
                Some(value) => value,
                None => Priority::DEFAULT,
            },
            match cancelable.clone() {
                Some(value) => Some(value),
                None => None::<Cancellable>,
            }
            .as_ref(),
            move |result| match result {
                Ok(bytes) => {
                    // No bytes were read, end of stream
                    if bytes.len() == 0 {
                        return callback(Ok(self));
                    }

                    // Save chunk to buffer
                    if let Err(reason) = self.push(bytes) {
                        return callback(Err((reason, None)));
                    };

                    // Continue bytes read..
                    self.read_all_async(connection, cancelable, priority, chunk, callback);
                }
                Err(reason) => callback(Err((Error::InputStream, Some(reason.message())))),
            },
        );
    }

    /// Push [Bytes](https://docs.gtk.org/glib/struct.Bytes.html) to `Self.buffer`
    ///
    /// Return `Error::Overflow` on `max_size` reached
    pub fn push(&mut self, bytes: Bytes) -> Result<usize, Error> {
        // Calculate new size value
        let total = self.buffer.len() + bytes.len();

        // Validate overflow
        if total > self.max_size {
            return Err(Error::Overflow);
        }

        // Success
        self.buffer.push(bytes);

        Ok(total)
    }

    // Setters

    /// Set new `max_size` value, `DEFAULT_MAX_SIZE` if `None`
    pub fn set_max_size(&mut self, value: Option<usize>) {
        self.max_size = match value {
            Some(size) => size,
            None => DEFAULT_MAX_SIZE,
        }
    }

    // Getters

    /// Get reference to bytes collected
    pub fn buffer(&self) -> &Vec<Bytes> {
        &self.buffer
    }

    /// Return copy of bytes as UTF-8 vector
    pub fn to_utf8(&self) -> Vec<u8> {
        self.buffer
            .iter()
            .flat_map(|byte| byte.iter())
            .cloned()
            .collect()
    }
}
