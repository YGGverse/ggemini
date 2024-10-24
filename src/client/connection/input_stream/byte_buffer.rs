pub mod error;

pub use error::Error;

use gio::{prelude::InputStreamExt, Cancellable, InputStream};
use glib::{object::IsA, Bytes, Priority};

pub const DEFAULT_CAPACITY: usize = 0x400;
pub const DEFAULT_CHUNK_SIZE: usize = 0x100;
pub const DEFAULT_MAX_SIZE: usize = 0xfffff;

pub struct ByteBuffer {
    bytes: Vec<Bytes>,
}

impl ByteBuffer {
    // Constructors

    /// Create new dynamically allocated bytes buffer with default capacity
    pub fn new() -> Self {
        Self::with_capacity(Some(DEFAULT_CAPACITY))
    }

    /// Create new dynamically allocated bytes buffer with initial capacity
    ///
    /// Options:
    /// * initial bytes request to reduce extra memory overwrites (1024 by default)
    pub fn with_capacity(value: Option<usize>) -> Self {
        Self {
            bytes: Vec::with_capacity(match value {
                Some(capacity) => capacity,
                None => DEFAULT_CAPACITY,
            }),
        }
    }

    // Readers

    /// Populate bytes buffer synchronously from `gio::InputStream`
    ///
    /// Options:
    /// * `input_stream` https://docs.gtk.org/gio/class.InputStream.html
    /// * `cancellable` https://docs.gtk.org/gio/class.Cancellable.html
    /// * `chunk_size` bytes limit to read per iter (256 by default)
    /// * `max_size` bytes limit to prevent memory overflow (1M by default)
    pub fn read_input_stream(
        mut self,
        input_stream: InputStream,
        cancellable: Option<&impl IsA<Cancellable>>,
        chunk_size: Option<usize>,
        max_size: Option<usize>,
    ) -> Result<Self, Error> {
        // Disallow unlimited buffer, use defaults on None
        let limit = match max_size {
            Some(value) => value,
            None => DEFAULT_MAX_SIZE,
        };

        loop {
            // Check buffer size to prevent memory overflow
            if self.bytes.len() > limit {
                return Err(Error::Overflow);
            }

            // Continue bytes reading
            match input_stream.read_bytes(
                match chunk_size {
                    Some(value) => value,
                    None => DEFAULT_CHUNK_SIZE,
                },
                cancellable,
            ) {
                Ok(bytes) => {
                    // No bytes were read, end of stream
                    if bytes.len() == 0 {
                        return Ok(self);
                    }

                    // Save chunk to buffer
                    self.bytes.push(bytes);
                }
                Err(_) => return Err(Error::StreamChunkRead),
            };
        }
    }

    /// Populate bytes buffer asynchronously from `gio::InputStream`,
    /// apply callback function to `Ok(Self)` on success
    ///
    /// Options:
    /// * `input_stream` https://docs.gtk.org/gio/class.InputStream.html
    /// * `cancellable` https://docs.gtk.org/gio/class.Cancellable.html
    /// * `priority` e.g. https://docs.gtk.org/glib/const.PRIORITY_DEFAULT.html
    /// * `chunk_size` optional bytes limit to read per iter (256 by default)
    /// * `max_size` optional bytes limit to prevent memory overflow (1M by default)
    /// * `callback` user function to apply on complete
    pub fn read_input_stream_async(
        mut self,
        input_stream: InputStream,
        cancellable: Cancellable,
        priority: Priority,
        chunk_size: Option<usize>,
        max_size: Option<usize>,
        callback: impl FnOnce(Result<Self, Error>) + 'static,
    ) {
        // Clone reference counted chunk dependencies
        let _input_stream = input_stream.clone();
        let _cancellable = cancellable.clone();

        // Continue bytes reading
        input_stream.read_bytes_async(
            match max_size {
                Some(value) => value,
                None => DEFAULT_MAX_SIZE,
            },
            priority,
            Some(&cancellable),
            move |result| -> () {
                match result {
                    Ok(bytes) => {
                        // No bytes were read, end of stream
                        if bytes.len() == 0 {
                            return callback(Ok(self));
                        }

                        // Save chunk to buffer
                        self.bytes.push(bytes);

                        // Continue bytes reading...
                        self.read_input_stream_async(
                            _input_stream,
                            _cancellable,
                            priority,
                            chunk_size,
                            max_size,
                            callback,
                        );
                    }
                    Err(_) => callback(Err(Error::StreamChunkReadAsync)),
                }
            },
        );
    }

    /// Get link to bytes collected
    pub fn bytes(&self) -> &Vec<Bytes> {
        &self.bytes
    }

    /// Return a copy of the bytes in UTF-8
    pub fn to_utf8(&self) -> Vec<u8> {
        self.bytes
            .iter()
            .flat_map(|byte| byte.iter())
            .cloned()
            .collect()
    }
}
