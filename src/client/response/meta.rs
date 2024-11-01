//! Components for reading and parsing meta bytes from response:
//! * [Gemini status code](https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes)
//! * meta data (for interactive statuses like 10, 11, 30 etc)
//! * MIME type

pub mod data;
pub mod error;
pub mod mime;
pub mod status;

pub use data::Data;
pub use error::Error;
pub use mime::Mime;
pub use status::Status;

use gio::{
    prelude::{IOStreamExt, InputStreamExtManual},
    Cancellable, SocketConnection,
};
use glib::Priority;

pub const MAX_LEN: usize = 0x400; // 1024

pub struct Meta {
    data: Data,
    mime: Option<Mime>,
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

        // Can parse from entire response or just meta buffer given
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

    pub fn mime(&self) -> &Option<Mime> {
        &self.mime
    }
}

// Tools

/// Asynchronously read all meta bytes from [InputStream](https://docs.gtk.org/gio/class.InputStream.html)
/// for given [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html)
///
/// Return UTF-8 buffer collected.
///
/// * this function implements low-level helper for `Meta::from_socket_connection_async`, also provides public API for external integrations
/// * requires `SocketConnection` instead of `InputStream` to keep connection alive (by increasing reference count in async context) @TODO
pub fn read_from_socket_connection_async(
    mut buffer: Vec<u8>,
    connection: SocketConnection,
    cancellable: Option<Cancellable>,
    priority: Priority,
    on_complete: impl FnOnce(Result<Vec<u8>, (Error, Option<&str>)>) + 'static,
) {
    connection.input_stream().read_async(
        vec![0],
        priority,
        cancellable.clone().as_ref(),
        move |result| match result {
            Ok((mut bytes, size)) => {
                // Expect valid header length
                if size == 0 || buffer.len() >= MAX_LEN {
                    return on_complete(Err((Error::Protocol, None)));
                }

                // Read next byte without record
                if bytes.contains(&b'\r') {
                    return read_from_socket_connection_async(
                        buffer,
                        connection,
                        cancellable,
                        priority,
                        on_complete,
                    );
                }

                // Complete without record
                if bytes.contains(&b'\n') {
                    return on_complete(Ok(buffer));
                }

                // Record
                buffer.append(&mut bytes);

                // Continue
                read_from_socket_connection_async(
                    buffer,
                    connection,
                    cancellable,
                    priority,
                    on_complete,
                );
            }
            Err((_, reason)) => on_complete(Err((Error::InputStream, Some(reason.message())))),
        },
    );
}
