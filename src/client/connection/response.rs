pub mod certificate;
pub mod error;
pub mod failure;
pub mod input;
pub mod redirect;
pub mod success;

pub use certificate::Certificate;
pub use error::{Error, HeaderBytesError};
pub use failure::Failure;
pub use input::Input;
pub use redirect::Redirect;
pub use success::Success;

use super::Connection;
use gio::{Cancellable, IOStream};
use glib::{Priority, object::IsA};

const HEADER_LEN: usize = 1024;

/// https://geminiprotocol.net/docs/protocol-specification.gmi#responses
pub enum Response {
    Input(Input),             // 1*
    Success(Success),         // 2*
    Redirect(Redirect),       // 3*
    Failure(Failure),         // 4*,5*
    Certificate(Certificate), // 6*
}

impl Response {
    /// Asynchronously create new `Self` for given `Connection`
    pub fn header_from_connection_async(
        connection: Connection,
        priority: Priority,
        cancellable: Cancellable,
        callback: impl FnOnce(Result<Self, Error>, Connection) + 'static,
    ) {
        header_from_stream_async(
            Vec::with_capacity(HEADER_LEN),
            connection.stream(),
            cancellable,
            priority,
            |result| {
                callback(
                    match result {
                        Ok(buffer) => match buffer.first() {
                            Some(b) => match b {
                                b'1' => match Input::from_utf8(&buffer) {
                                    Ok(input) => Ok(Self::Input(input)),
                                    Err(e) => Err(Error::Input(e)),
                                },
                                b'2' => match Success::from_utf8(&buffer) {
                                    Ok(success) => Ok(Self::Success(success)),
                                    Err(e) => Err(Error::Success(e)),
                                },
                                b'3' => match Redirect::from_utf8(&buffer) {
                                    Ok(redirect) => Ok(Self::Redirect(redirect)),
                                    Err(e) => Err(Error::Redirect(e)),
                                },
                                b'4' | b'5' => match Failure::from_utf8(&buffer) {
                                    Ok(failure) => Ok(Self::Failure(failure)),
                                    Err(e) => Err(Error::Failure(e)),
                                },
                                b'6' => match Certificate::from_utf8(&buffer) {
                                    Ok(certificate) => Ok(Self::Certificate(certificate)),
                                    Err(e) => Err(Error::Certificate(e)),
                                },
                                b => Err(Error::Code(*b)),
                            },
                            None => Err(Error::Protocol(buffer)),
                        },
                        Err(e) => Err(e),
                    },
                    connection,
                )
            },
        );
    }
}

// Tools

/// Asynchronously read header bytes from [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
///
/// Return UTF-8 buffer collected
/// * requires `IOStream` reference to keep `Connection` active in async thread
fn header_from_stream_async(
    mut buffer: Vec<u8>,
    stream: impl IsA<IOStream>,
    cancellable: Cancellable,
    priority: Priority,
    callback: impl FnOnce(Result<Vec<u8>, Error>) + 'static,
) {
    use gio::prelude::{IOStreamExt, InputStreamExtManual};
    stream.input_stream().read_async(
        vec![0],
        priority,
        Some(&cancellable.clone()),
        move |result| match result {
            Ok((bytes, size)) => {
                if size == 0 {
                    return callback(Ok(buffer));
                }
                if buffer.len() + bytes.len() > HEADER_LEN {
                    buffer.extend(bytes);
                    return callback(Err(Error::Protocol(buffer)));
                }
                if bytes[0] == b'\r' {
                    buffer.extend(bytes);
                    return header_from_stream_async(
                        buffer,
                        stream,
                        cancellable,
                        priority,
                        callback,
                    );
                }
                if bytes[0] == b'\n' {
                    buffer.extend(bytes);
                    return callback(Ok(buffer));
                }
                buffer.extend(bytes);
                header_from_stream_async(buffer, stream, cancellable, priority, callback)
            }
            Err((data, e)) => callback(Err(Error::Stream(e, data))),
        },
    )
}

/// Get header bytes slice
/// * common for all child parsers
fn header_bytes(buffer: &[u8]) -> Result<&[u8], HeaderBytesError> {
    for (i, b) in buffer.iter().enumerate() {
        if i > 1024 {
            return Err(HeaderBytesError::Len);
        }
        if *b == b'\r' {
            let n = i + 1;
            if buffer.get(n).is_some_and(|b| *b == b'\n') {
                return Ok(&buffer[..n]);
            }
            break;
        }
    }
    Err(HeaderBytesError::End)
}
