//! Read and parse Gemini response as Object

pub mod data;
pub mod error;
pub mod meta;

pub use error::Error;
pub use meta::Meta;

use super::Connection;
use gio::Cancellable;
use glib::Priority;
use std::rc::Rc;

pub struct Response {
    pub connection: Rc<Connection>,
    pub meta: Meta,
}

impl Response {
    // Constructors

    pub fn from_request_async(
        connection: Rc<Connection>,
        priority: Option<Priority>,
        cancellable: Option<Cancellable>,
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
