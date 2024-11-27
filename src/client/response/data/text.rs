//! Tools for Text-based response

pub mod error;
pub use error::Error;

// Local dependencies
use gio::{
    prelude::{IOStreamExt, InputStreamExt},
    Cancellable, IOStream,
};
use glib::{object::IsA, GString, Priority};

// Default limits
pub const BUFFER_CAPACITY: usize = 0x400; // 1024
pub const BUFFER_MAX_SIZE: usize = 0xfffff; // 1M

/// Container for text-based response data
pub struct Text {
    pub data: GString,
}

impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
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
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        match GString::from_utf8(buffer.into()) {
            Ok(data) => Ok(Self::from_string(&data)),
            Err(reason) => Err(Error::Decode(reason)),
        }
    }

    /// Asynchronously create new `Self` from [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
    pub fn from_stream_async(
        stream: impl IsA<IOStream>,
        priority: Option<Priority>,
        cancellable: Option<Cancellable>,
        on_complete: impl FnOnce(Result<Self, Error>) + 'static,
    ) {
        read_all_from_stream_async(
            Vec::with_capacity(BUFFER_CAPACITY),
            stream,
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
}

// Tools

/// Asynchronously read all bytes from [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
///
/// Return UTF-8 buffer collected
/// * require `IOStream` reference to keep `Connection` active in async thread
pub fn read_all_from_stream_async(
    mut buffer: Vec<u8>,
    stream: impl IsA<IOStream>,
    cancelable: Option<Cancellable>,
    priority: Priority,
    callback: impl FnOnce(Result<Vec<u8>, Error>) + 'static,
) {
    stream.input_stream().read_bytes_async(
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
                    return callback(Err(Error::BufferOverflow));
                }

                // Save chunks to buffer
                for &byte in bytes.iter() {
                    buffer.push(byte);
                }

                // Continue bytes reading
                read_all_from_stream_async(buffer, stream, cancelable, priority, callback);
            }
            Err(reason) => callback(Err(Error::InputStream(reason))),
        },
    );
}
