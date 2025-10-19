pub mod error;
pub mod size;

pub use error::Error;
pub use size::Size;

use gio::{
    Cancellable, FileOutputStream, IOStream,
    prelude::{IOStreamExt, InputStreamExt, OutputStreamExtManual},
};
use glib::{Bytes, Priority, object::IsA};

/// Asynchronously move all bytes from [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
/// to [FileOutputStream](https://docs.gtk.org/gio/class.FileOutputStream.html)
/// * require `IOStream` reference to keep `Connection` active in async thread
pub fn from_stream_async(
    io_stream: impl IsA<IOStream>,
    file_output_stream: FileOutputStream,
    cancellable: Cancellable,
    priority: Priority,
    mut size: Size,
    (on_chunk, on_complete): (
        impl Fn(Bytes, usize) + 'static, // on_chunk
        impl FnOnce(Result<(FileOutputStream, usize), Error>) + 'static, // on_complete
    ),
) {
    io_stream.input_stream().read_bytes_async(
        size.chunk,
        priority,
        Some(&cancellable.clone()),
        move |result| match result {
            Ok(bytes) => {
                size.total += bytes.len();
                on_chunk(bytes.clone(), size.total);

                if let Some(limit) = size.limit
                    && size.total > limit
                {
                    return on_complete(Err(Error::BytesTotal(size.total, limit)));
                }

                if bytes.is_empty() {
                    return on_complete(Ok((file_output_stream, size.total)));
                }

                // Make sure **all bytes** sent to the destination
                // > A partial write is performed with the size of a message block, which is 16kB
                // > https://docs.openssl.org/3.0/man3/SSL_write/#notes
                file_output_stream.clone().write_all_async(
                    bytes,
                    priority,
                    Some(&cancellable.clone()),
                    move |result| match result {
                        Ok(_) => from_stream_async(
                            io_stream,
                            file_output_stream,
                            cancellable,
                            priority,
                            size,
                            (on_chunk, on_complete),
                        ),
                        Err((b, e)) => on_complete(Err(Error::OutputStream(b, e))),
                    },
                )
            }
            Err(e) => on_complete(Err(Error::InputStream(e))),
        },
    )
}
