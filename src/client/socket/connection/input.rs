pub mod buffer;
pub mod error;

pub use buffer::Buffer;
pub use error::Error;

use gio::{prelude::InputStreamExt, Cancellable, InputStream};
use glib::Priority;

pub const DEFAULT_READ_CHUNK: usize = 0x100;

pub struct Input {
    buffer: Buffer,
    stream: InputStream,
}

impl Input {
    // Constructors

    /// Create new `Input` from `gio::InputStream`
    ///
    /// https://docs.gtk.org/gio/class.InputStream.html
    pub fn new_from_stream(stream: InputStream) -> Self {
        Self {
            buffer: Buffer::new(),
            stream,
        }
    }

    // Actions

    /// Synchronously read all bytes from `gio::InputStream` to `input::Buffer`
    ///
    /// Return `Self` with `buffer` updated on success
    ///
    /// Options:
    /// * `cancellable` https://docs.gtk.org/gio/class.Cancellable.html
    /// * `chunk` max bytes to read per chunk (256 by default)
    pub fn read_all(
        mut self,
        cancelable: Option<Cancellable>,
        chunk: Option<usize>,
    ) -> Result<Self, Error> {
        loop {
            // Continue bytes reading
            match self.stream.read_bytes(
                match chunk {
                    Some(value) => value,
                    None => DEFAULT_READ_CHUNK,
                },
                match cancelable.clone() {
                    Some(value) => Some(value),
                    None => None::<Cancellable>,
                }
                .as_ref(),
            ) {
                Ok(bytes) => {
                    // No bytes were read, end of stream
                    if bytes.len() == 0 {
                        return Ok(self);
                    }

                    // Save chunk to buffer
                    match self.buffer.push(bytes) {
                        Ok(_) => continue,
                        Err(buffer::Error::Overflow) => return Err(Error::BufferOverflow),
                        Err(_) => return Err(Error::BufferWrite),
                    };
                }
                Err(_) => return Err(Error::StreamChunkRead),
            };
        }
    }

    /// Asynchronously read all bytes from `gio::InputStream` to `input::Buffer`
    ///
    /// * applies `callback` function on last byte reading complete;
    /// * return `Self` with `buffer` updated on success
    ///
    /// Options:
    /// * `cancellable` https://docs.gtk.org/gio/class.Cancellable.html (`None::<&Cancellable>` by default)
    /// * `priority` e.g. https://docs.gtk.org/glib/const.PRIORITY_DEFAULT.html (`Priority::DEFAULT` by default)
    /// * `chunk` optional max bytes to read per chunk (`DEFAULT_READ_CHUNK` by default)
    /// * `callback` user function to apply on async iteration complete or `None` to skip
    pub fn read_all_async(
        mut self,
        cancelable: Option<Cancellable>,
        priority: Option<Priority>,
        chunk: Option<usize>,
        callback: impl FnOnce(Result<Self, Error>) + 'static,
    ) {
        // Continue bytes reading
        self.stream.clone().read_bytes_async(
            match chunk {
                Some(value) => value,
                None => DEFAULT_READ_CHUNK,
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
            move |result| {
                match result {
                    Ok(bytes) => {
                        // No bytes were read, end of stream
                        if bytes.len() == 0 {
                            return callback(Ok(self));
                        }

                        // Save chunk to buffer
                        match self.buffer.push(bytes) {
                            Err(buffer::Error::Overflow) => {
                                return callback(Err(Error::BufferOverflow))
                            }

                            // Other errors related to write issues @TODO test
                            Err(_) => return callback(Err(Error::BufferWrite)),

                            // Async function, nothing to return yet
                            _ => (),
                        };

                        // Continue bytes reading...
                        self.read_all_async(cancelable, priority, chunk, callback);
                    }
                    Err(_) => callback(Err(Error::StreamChunkRead)),
                }
            },
        );
    }

    // Setters

    pub fn set_buffer(&mut self, buffer: Buffer) {
        self.buffer = buffer;
    }

    pub fn set_stream(&mut self, stream: InputStream) {
        self.stream = stream;
    }

    // Getters

    /// Get reference to `Buffer`
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Get reference to `gio::InputStream`
    pub fn stream(&self) -> &InputStream {
        &self.stream
    }
}
