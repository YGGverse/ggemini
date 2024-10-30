pub mod data;
pub mod error;
pub mod mime;
pub mod status;

pub use data::Data;
pub use error::Error;
pub use mime::Mime;
pub use status::Status;

use gio::{
    prelude::{IOStreamExt, InputStreamExt},
    Cancellable, SocketConnection,
};
use glib::{Bytes, Priority};

pub const MAX_LEN: usize = 0x400; // 1024

pub struct Meta {
    data: Data,
    mime: Mime,
    status: Status,
    // @TODO
    // charset: Charset,
    // language: Language,
}

impl Meta {
    // Constructors

    /// Create new `Self` from UTF-8 buffer
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, (Error, Option<&str>)> {
        let len = buffer.len();

        match buffer.get(..if len > MAX_LEN { MAX_LEN } else { len }) {
            Some(slice) => {
                // Parse data
                let data = Data::from_utf8(&slice);

                if let Err(reason) = data {
                    return Err((
                        match reason {
                            data::Error::Decode => Error::DataDecode,
                            data::Error::Protocol => Error::DataProtocol,
                        },
                        None,
                    ));
                }

                // MIME

                let mime = Mime::from_utf8(&slice);

                if let Err(reason) = mime {
                    return Err((
                        match reason {
                            mime::Error::Decode => Error::MimeDecode,
                            mime::Error::Protocol => Error::MimeProtocol,
                            mime::Error::Undefined => Error::MimeUndefined,
                        },
                        None,
                    ));
                }

                // Status

                let status = Status::from_utf8(&slice);

                if let Err(reason) = status {
                    return Err((
                        match reason {
                            status::Error::Decode => Error::StatusDecode,
                            status::Error::Protocol => Error::StatusProtocol,
                            status::Error::Undefined => Error::StatusUndefined,
                        },
                        None,
                    ));
                }

                Ok(Self {
                    data: data.unwrap(),
                    mime: mime.unwrap(),
                    status: status.unwrap(),
                })
            }
            None => Err((Error::Protocol, None)),
        }
    }

    /// Asynchronously create new `Self` from [InputStream](https://docs.gtk.org/gio/class.InputStream.html)
    /// for given [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html)
    pub fn from_socket_connection_async(
        socket_connection: SocketConnection,
        priority: Option<Priority>,
        cancellable: Option<Cancellable>,
        on_complete: impl FnOnce(Result<Self, (Error, Option<&str>)>) + 'static,
    ) {
        read_from_socket_connection_async(
            Vec::with_capacity(MAX_LEN),
            socket_connection,
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

    // Getters

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn data(&self) -> &Data {
        &self.data
    }

    pub fn mime(&self) -> &Mime {
        &self.mime
    }
}

// Tools

/// Asynchronously take meta bytes from [InputStream](https://docs.gtk.org/gio/class.InputStream.html)
/// for given [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html)
///
/// * this function implements low-level helper for `Meta::from_socket_connection_async`, also provides public API for external integrations
/// * requires entire `SocketConnection` instead of `InputStream` to keep connection alive in async context
pub fn read_from_socket_connection_async(
    mut buffer: Vec<Bytes>,
    connection: SocketConnection,
    cancellable: Option<Cancellable>,
    priority: Priority,
    on_complete: impl FnOnce(Result<Vec<u8>, (Error, Option<&str>)>) + 'static,
) {
    connection.input_stream().read_bytes_async(
        1, // do not change!
        priority,
        cancellable.clone().as_ref(),
        move |result| match result {
            Ok(bytes) => {
                // Expect valid header length
                if bytes.len() == 0 || buffer.len() >= MAX_LEN {
                    return on_complete(Err((Error::Protocol, None)));
                }

                // Read next byte without buffer record
                if bytes.contains(&b'\r') {
                    return read_from_socket_connection_async(
                        buffer,
                        connection,
                        cancellable,
                        priority,
                        on_complete,
                    );
                }

                // Complete without buffer record
                if bytes.contains(&b'\n') {
                    return on_complete(Ok(buffer
                        .iter()
                        .flat_map(|byte| byte.iter())
                        .cloned()
                        .collect())); // convert to UTF-8
                }

                // Record
                buffer.push(bytes);

                // Continue
                read_from_socket_connection_async(
                    buffer,
                    connection,
                    cancellable,
                    priority,
                    on_complete,
                );
            }
            Err(reason) => on_complete(Err((Error::InputStream, Some(reason.message())))),
        },
    );
}
