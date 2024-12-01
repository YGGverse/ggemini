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
    pub status: Status,
    pub data: Option<Data>,
    pub mime: Option<Mime>,
    // @TODO
    // charset: Option<Charset>,
    // language: Option<Language>,
}

impl Meta {
    // Constructors

    /// Create new `Self` from UTF-8 buffer
    /// * supports entire response or just meta slice
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        // Calculate buffer length once
        let len = buffer.len();

        // Parse meta bytes only
        match buffer.get(..if len > MAX_LEN { MAX_LEN } else { len }) {
            Some(slice) => {
                // Parse data
                let data = Data::from_utf8(slice);

                if let Err(e) = data {
                    return Err(Error::Data(e));
                }

                // MIME

                let mime = Mime::from_utf8(slice);

                if let Err(e) = mime {
                    return Err(Error::Mime(e));
                }

                // Status

                let status = Status::from_utf8(slice);

                if let Err(e) = status {
                    return Err(Error::Status(e));
                }

                Ok(Self {
                    data: data.unwrap(),
                    mime: mime.unwrap(),
                    status: status.unwrap(),
                })
            }
            None => Err(Error::Protocol),
        }
    }

    /// Asynchronously create new `Self` from [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
    pub fn from_stream_async(
        stream: impl IsA<IOStream>,
        priority: Priority,
        cancellable: Cancellable,
        on_complete: impl FnOnce(Result<Self, Error>) + 'static,
    ) {
        read_from_stream_async(
            Vec::with_capacity(MAX_LEN),
            stream,
            cancellable,
            priority,
            |result| match result {
                Ok(buffer) => on_complete(Self::from_utf8(&buffer)),
                Err(e) => on_complete(Err(e)),
            },
        );
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
    cancellable: Cancellable,
    priority: Priority,
    on_complete: impl FnOnce(Result<Vec<u8>, Error>) + 'static,
) {
    stream.input_stream().read_async(
        vec![0],
        priority,
        Some(&cancellable.clone()),
        move |result| match result {
            Ok((mut bytes, size)) => {
                // Expect valid header length
                if size == 0 || buffer.len() >= MAX_LEN {
                    return on_complete(Err(Error::Protocol));
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
            Err((data, e)) => on_complete(Err(Error::InputStream(data, e))),
        },
    );
}
