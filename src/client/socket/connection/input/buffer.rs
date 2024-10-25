pub mod error;
pub use error::Error;

use glib::Bytes;

pub const DEFAULT_CAPACITY: usize = 0x400;
pub const DEFAULT_MAX_SIZE: usize = 0xfffff;

pub struct Buffer {
    bytes: Vec<Bytes>,
    max_size: usize,
}

impl Buffer {
    // Constructors

    /// Create new dynamically allocated `Buffer` with default `capacity` and `max_size` limit
    pub fn new() -> Self {
        Self::new_with_options(Some(DEFAULT_CAPACITY), Some(DEFAULT_MAX_SIZE))
    }

    /// Create new dynamically allocated `Buffer` with options
    ///
    /// Options:
    /// * `capacity` initial bytes request to reduce extra memory overwrites (1024 by default)
    /// * `max_size` max bytes to prevent memory overflow (1M by default)
    pub fn new_with_options(capacity: Option<usize>, max_size: Option<usize>) -> Self {
        Self {
            bytes: Vec::with_capacity(match capacity {
                Some(value) => value,
                None => DEFAULT_CAPACITY,
            }),
            max_size: match max_size {
                Some(value) => value,
                None => DEFAULT_MAX_SIZE,
            },
        }
    }

    // Setters

    /// Set new `Buffer.max_size` value to prevent memory overflow
    ///
    /// Use `DEFAULT_MAX_SIZE` if `None` given.
    pub fn set_max_size(&mut self, value: Option<usize>) {
        self.max_size = match value {
            Some(size) => size,
            None => DEFAULT_MAX_SIZE,
        }
    }

    // Actions

    /// Push `glib::Bytes` to `Buffer.bytes`
    ///
    /// Return `Error::Overflow` on `Buffer.max_size` reached.
    pub fn push(&mut self, bytes: Bytes) -> Result<usize, Error> {
        // Calculate new size value
        let total = self.bytes.len() + bytes.len();

        // Validate overflow
        if total > self.max_size {
            return Err(Error::Overflow);
        }

        // Success
        self.bytes.push(bytes);

        Ok(total)
    }

    // Getters

    /// Get reference to bytes collected
    pub fn bytes(&self) -> &Vec<Bytes> {
        &self.bytes
    }

    /// Return copy of bytes as UTF-8 vector
    pub fn to_utf8(&self) -> Vec<u8> {
        self.bytes
            .iter()
            .flat_map(|byte| byte.iter())
            .cloned()
            .collect()
    }
}
