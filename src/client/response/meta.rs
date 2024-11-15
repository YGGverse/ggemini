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
    Cancellable, IOStream,
};
use glib::{object::IsA, Priority};

pub const MAX_LEN: usize = 0x400; // 1024

pub struct Meta {
    status: Status,
    data: Option<Data>,
    mime: Option<Mime>,
    // @TODO
    // charset: Option<Charset>,
    // language: Option<Language>,
}

impl Meta {
    // Constructors

    /// Create new `Self` from UTF-8 buffer
    /// * supports entire response or just meta slice
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, (Error, Option<&str>)> {
        // Calculate buffer length once
        let len = buffer.len();

        // Parse meta bytes only
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

    /// Asynchronously create new `Self` from [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
    pub fn from_stream_async(
        stream: impl IsA<IOStream>,
        priority: Option<Priority>,
        cancellable: Option<Cancellable>,
        on_complete: impl FnOnce(Result<Self, (Error, Option<&str>)>) + 'static,
    ) {
        read_from_stream_async(
            Vec::with_capacity(MAX_LEN),
            stream,
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

    pub fn data(&self) -> &Option<Data> {
        &self.data
    }

    pub fn mime(&self) -> &Option<Mime> {
        &self.mime
    }
}

// Tools

/// Asynchronously read all meta bytes from [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
///
/// Return UTF-8 buffer collected
/// * require `IOStream` reference to keep `Connection` active in async thread
pub fn read_from_stream_async(
    mut buffer: Vec<u8>,
    stream: impl IsA<IOStream>,
    cancellable: Option<Cancellable>,
    priority: Priority,
    on_complete: impl FnOnce(Result<Vec<u8>, (Error, Option<&str>)>) + 'static,
) {
    stream.input_stream().read_async(
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
                    return read_from_stream_async(
                        buffer,
                        stream,
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
                read_from_stream_async(buffer, stream, cancellable, priority, on_complete);
            }
            Err((_, reason)) => on_complete(Err((Error::InputStream, Some(reason.message())))),
        },
    );
}
