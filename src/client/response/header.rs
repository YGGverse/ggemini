pub mod error;
pub mod meta;
pub mod mime;
pub mod status;

pub use error::Error;
pub use meta::Meta;
pub use mime::Mime;
pub use status::Status;

use gio::{
    prelude::{IOStreamExt, InputStreamExt},
    Cancellable, SocketConnection,
};
use glib::{Bytes, Priority};

pub const HEADER_BYTES_LEN: usize = 0x400; // 1024

pub struct Header {
    status: Status,
    meta: Option<Meta>,
    mime: Option<Mime>,
    // @TODO
    // charset: Option<Charset>,
    // language: Option<Language>,
}

impl Header {
    // Constructors

    pub fn from_socket_connection_async(
        socket_connection: SocketConnection,
        priority: Option<Priority>,
        cancellable: Option<Cancellable>,
        callback: impl FnOnce(Result<Self, (Error, Option<&str>)>) + 'static,
    ) {
        // Take header buffer from input stream
        Self::read_from_socket_connection_async(
            Vec::with_capacity(HEADER_BYTES_LEN),
            socket_connection,
            match cancellable {
                Some(value) => Some(value),
                None => None::<Cancellable>,
            },
            match priority {
                Some(value) => value,
                None => Priority::DEFAULT,
            },
            |result| {
                callback(match result {
                    Ok(buffer) => {
                        // Status is required, parse to continue
                        match Status::from_header(&buffer) {
                            Ok(status) => Ok(Self {
                                status,
                                meta: match Meta::from_header(&buffer) {
                                    Ok(meta) => Some(meta),
                                    Err(_) => None,
                                },
                                mime: match Mime::from_header(&buffer) {
                                    Ok(mime) => Some(mime),
                                    Err(_) => None,
                                },
                            }),
                            Err(reason) => Err((
                                match reason {
                                    status::Error::Decode => Error::StatusDecode,
                                    status::Error::Undefined => Error::StatusUndefined,
                                    status::Error::Protocol => Error::StatusProtocol,
                                },
                                None,
                            )),
                        }
                    }
                    Err(error) => Err(error),
                })
            },
        );
    }

    // Getters

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn mime(&self) -> &Option<Mime> {
        &self.mime
    }

    pub fn meta(&self) -> &Option<Meta> {
        &self.meta
    }

    // Tools

    pub fn read_from_socket_connection_async(
        mut buffer: Vec<Bytes>,
        connection: SocketConnection,
        cancellable: Option<Cancellable>,
        priority: Priority,
        callback: impl FnOnce(Result<Vec<u8>, (Error, Option<&str>)>) + 'static,
    ) {
        connection.input_stream().read_bytes_async(
            1, // do not change!
            priority,
            cancellable.clone().as_ref(),
            move |result| match result {
                Ok(bytes) => {
                    // Expect valid header length
                    if bytes.len() == 0 || buffer.len() >= HEADER_BYTES_LEN {
                        return callback(Err((Error::Protocol, None)));
                    }

                    // Read next byte without buffer record
                    if bytes.contains(&b'\r') {
                        return Self::read_from_socket_connection_async(
                            buffer,
                            connection,
                            cancellable,
                            priority,
                            callback,
                        );
                    }

                    // Complete without buffer record
                    if bytes.contains(&b'\n') {
                        return callback(Ok(buffer
                            .iter()
                            .flat_map(|byte| byte.iter())
                            .cloned()
                            .collect())); // convert to UTF-8
                    }

                    // Record
                    buffer.push(bytes);

                    // Continue
                    Self::read_from_socket_connection_async(
                        buffer,
                        connection,
                        cancellable,
                        priority,
                        callback,
                    );
                }
                Err(reason) => callback(Err((Error::InputStream, Some(reason.message())))),
            },
        );
    }
}
