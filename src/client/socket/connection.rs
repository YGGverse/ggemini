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

    pub fn new_from(connection: SocketConnection) -> Self {
        Self { connection }
    }

    // Actions

    /// MIddle-level API to make single async socket request for current connection:
    ///
    /// 1. send `Uri` request to the ouput stream;
    /// 2. write entire input stream into the new `Vec<u8>` buffer on success;
    /// 3. close current connection if callback function does not prevent that action by return.
    pub fn request_async(
        self,
        uri: Uri,
        cancelable: Option<Cancellable>,
        priority: Option<Priority>,
        chunk: Option<usize>,
        callback: impl FnOnce(Result<Vec<u8>, Error>) + 'static,
    ) {
        // Send request
        Output::new_from_stream(self.connection.output_stream()).write_async(
            &Bytes::from(gformat!("{}\r\n", uri.to_str()).as_bytes()),
            cancelable.clone(),
            priority,
            move |output| match output {
                Ok(_) => {
                    // Read response
                    Input::new_from_stream(self.connection.input_stream()).read_all_async(
                        cancelable.clone(),
                        priority,
                        chunk,
                        move |input| {
                            // Apply callback function
                            callback(match input {
                                Ok(buffer) => Ok(buffer.to_utf8()),
                                Err(error) => Err(match error {
                                    input::Error::BufferOverflow => Error::InputBufferOverflow,
                                    input::Error::BufferWrite => Error::InputBufferWrite,
                                    input::Error::StreamChunkRead => Error::InputStreamChunkRead,
                                }),
                            });

                            // Close connection if callback act does not prevent that
                            self.close_async(cancelable, priority, |_| {}); // @TODO
                        },
                    );
                }
                Err(error) => {
                    // Apply callback function
                    callback(Err(match error {
                        output::Error::StreamWrite => Error::OutputStreamWrite,
                    }));

                    // Close connection if callback act does not prevent that
                    self.close_async(cancelable, priority, |_| {}); // @TODO
                }
            },
        );
    }

    /// Asynchronously close current connection
    ///
    /// Options:
    /// * `cancellable` https://docs.gtk.org/gio/class.Cancellable.html (`None::<&Cancellable>` by default)
    /// * `priority` e.g. https://docs.gtk.org/glib/const.PRIORITY_DEFAULT.html (`Priority::DEFAULT` by default)
    /// * `callback` function to apply on complete
    pub fn close_async(
        self,
        cancelable: Option<Cancellable>,
        priority: Option<Priority>,
        callback: impl FnOnce(Result<(), Error>) + 'static,
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
                callback(match result {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Error::Close),
                })
            },
        );
    }

    // Getters

    pub fn connection(&self) -> &SocketConnection {
        &self.connection
    }
}
