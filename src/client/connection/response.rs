//! Read and parse Gemini response as Object

pub mod data;
pub mod error;
pub mod meta;

pub use error::Error;
pub use meta::Meta;

use super::Connection;
use gio::Cancellable;
use glib::Priority;

pub struct Response {
    pub connection: Connection,
    pub meta: Meta,
}

impl Response {
    // Constructors

    /// Create new `Self` from given `Connection`
    /// * useful for manual [IOStream](https://docs.gtk.org/gio/class.IOStream.html) handle (based on `Meta` bytes pre-parsed)
    pub fn from_connection_async(
        connection: Connection,
        priority: Priority,
        cancellable: Cancellable,
        callback: impl FnOnce(Result<Self, Error>) + 'static,
    ) {
        Meta::from_stream_async(connection.stream(), priority, cancellable, |result| {
            callback(match result {
                Ok(meta) => Ok(Self { connection, meta }),
                Err(e) => Err(Error::Meta(e)),
            })
        })
    }
}
