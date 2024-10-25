pub mod error;

pub use error::Error;

use gio::{prelude::OutputStreamExt, Cancellable, OutputStream};
use glib::{Bytes, Priority};

pub struct Output {
    stream: OutputStream,
}

impl Output {
    // Constructors

    /// Create new `Output` from `gio::OutputStream`
    ///
    /// https://docs.gtk.org/gio/class.OutputStream.html
    pub fn new_from_stream(stream: OutputStream) -> Self {
        Self { stream }
    }

    // Actions

    /// Asynchronously write all bytes to `gio::OutputStream`,
    ///
    /// applies `callback` function on last byte sent.
    ///
    /// Options:
    /// * `cancellable` https://docs.gtk.org/gio/class.Cancellable.html (`None::<&Cancellable>` by default)
    /// * `priority` e.g. https://docs.gtk.org/glib/const.PRIORITY_DEFAULT.html (`Priority::DEFAULT` by default)
    /// * `callback` user function to apply on complete
    pub fn write_async(
        &self,
        bytes: &Bytes,
        cancelable: Option<Cancellable>,
        priority: Option<Priority>,
        callback: impl FnOnce(Result<isize, Error>) + 'static,
    ) {
        self.stream.write_bytes_async(
            bytes,
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
                callback(match result {
                    Ok(size) => Ok(size),
                    Err(_) => Err(Error::StreamWrite),
                })
            },
        );
    }

    // Setters

    pub fn set_stream(&mut self, stream: OutputStream) {
        self.stream = stream;
    }

    // Getters

    /// Get reference to `gio::OutputStream`
    pub fn stream(&self) -> &OutputStream {
        &self.stream
    }
}
