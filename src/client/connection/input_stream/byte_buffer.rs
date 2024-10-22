pub mod error;

pub use error::Error;

use gio::{prelude::InputStreamExt, Cancellable, InputStream};
use glib::{object::IsA, Bytes};

pub const DEFAULT_CAPACITY: usize = 0x400;
pub const DEFAULT_CHUNK_SIZE: usize = 0x100;
pub const DEFAULT_MAX_SIZE: usize = 0xfffff;

pub struct ByteBuffer {
    bytes: Vec<Bytes>,
}

impl ByteBuffer {
    /// Create dynamically allocated bytes buffer from `gio::InputStream`
    ///
    /// Options:
    /// * `capacity` bytes request to reduce extra memory overwrites (1024 by default)
    /// * `chunk_size` bytes limit to read per iter (256 by default)
    /// * `max_size` bytes limit to prevent memory overflow (1M by default)
    pub fn from_input_stream(
        input_stream: &InputStream, // @TODO
        cancellable: Option<&impl IsA<Cancellable>>,
        capacity: Option<usize>,
        chunk_size: Option<usize>,
        max_size: Option<usize>,
    ) -> Result<Self, Error> {
        // Create buffer with initial capacity
        let mut buffer: Vec<Bytes> = Vec::with_capacity(match capacity {
            Some(value) => value,
            None => DEFAULT_CAPACITY,
        });

        // Disallow unlimited buffer, use defaults on None
        let limit = match max_size {
            Some(value) => value,
            None => DEFAULT_MAX_SIZE,
        };

        loop {
            // Check buffer size to prevent memory overflow
            if buffer.len() > limit {
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
                        break;
                    }

                    // Save chunk to buffer
                    buffer.push(bytes);
                }
                Err(_) => return Err(Error::Stream),
            };
        }

        // Done
        Ok(Self { bytes: buffer })
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
