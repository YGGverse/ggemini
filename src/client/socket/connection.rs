pub mod error;
pub mod input;
pub mod output;

pub use error::Error;
pub use input::Input;
pub use output::Output;

use gio::{prelude::IOStreamExt, Cancellable, SocketConnection};
use glib::{gformat, Bytes, Priority, Uri};

pub struct Connection {
    connection: SocketConnection,
}

impl Connection {
    // Constructors

    /// Create new `Self` from [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html)
    pub fn new_from(connection: SocketConnection) -> Self {
        Self { connection }
    }

    // Actions

    /// Middle-level API to make async socket request for current connection:
    ///
    /// 1. send request for [Uri](https://docs.gtk.org/glib/struct.Uri.html)
    ///    to the ouput [OutputStream](https://docs.gtk.org/gio/class.OutputStream.html);
    /// 2. write entire [InputStream](https://docs.gtk.org/gio/class.InputStream.html)
    ///    into `Vec<u8>` buffer on success;
    /// 3. return taken `Self` with `Result(Vec<u8>, Error)` on complete.
    pub fn request_async(
        self,
        uri: Uri,
        cancelable: Option<Cancellable>,
        priority: Option<Priority>,
        chunk: Option<usize>,
        callback: impl FnOnce(Self, Result<Vec<u8>, Error>) + 'static,
    ) {
        Output::new_from_stream(self.connection.output_stream()).write_async(
            &Bytes::from(gformat!("{}\r\n", uri.to_str()).as_bytes()),
            cancelable.clone(),
            priority,
            move |output| match output {
                Ok(_) => {
                    Input::new_from_stream(self.connection.input_stream()).read_all_async(
                        cancelable.clone(),
                        priority,
                        chunk,
                        move |this, input| {
                            callback(
                                self,
                                match input {
                                    Ok(()) => Ok(this.buffer().to_utf8()),
                                    Err(error) => Err(match error {
                                        input::Error::BufferOverflow => Error::InputBufferOverflow,
                                        input::Error::BufferWrite => Error::InputBufferWrite,
                                        input::Error::StreamChunkRead => {
                                            Error::InputStreamChunkRead
                                        }
                                    }),
                                },
                            );
                        },
                    );
                }
                Err(error) => {
                    callback(
                        self,
                        Err(match error {
                            output::Error::StreamWrite => Error::OutputStreamWrite,
                        }),
                    );
                }
            },
        );
    }

    /// Asynchronously close current connection
    ///
    /// Options:
    /// * `cancellable` see [Cancellable](https://docs.gtk.org/gio/class.Cancellable.html) (`None::<&Cancellable>` by default)
    /// * `priority` [Priority::DEFAULT](https://docs.gtk.org/glib/const.PRIORITY_DEFAULT.html) by default
    /// * `callback` optional function to apply on complete or `None` to skip
    pub fn close_async(
        &self,
        cancelable: Option<Cancellable>,
        priority: Option<Priority>,
        callback: Option<impl FnOnce(Result<(), Error>) + 'static>,
    ) {
        self.connection.close_async(
            match priority {
                Some(value) => value,
                None => Priority::DEFAULT,
            },
            match cancelable.clone() {
                Some(value) => Some(value),
                None => None::<Cancellable>,
            }
            .as_ref(),
            |result| {
                if let Some(call) = callback {
                    call(match result {
                        Ok(_) => Ok(()),
                        Err(_) => Err(Error::Close),
                    })
                }
            },
        );
    }

    // Getters

    /// Get reference to `gio::SocketConnection`
    ///
    /// https://docs.gtk.org/gio/class.SocketConnection.html
    pub fn connection(&self) -> &SocketConnection {
        &self.connection
    }
}
