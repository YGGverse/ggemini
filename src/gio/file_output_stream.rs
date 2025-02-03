pub mod error;
pub use error::Error;

use gio::{
    prelude::{IOStreamExt, InputStreamExt, OutputStreamExtManual},
    Cancellable, FileOutputStream, IOStream,
};
use glib::{object::IsA, Bytes, Priority};

/// Asynchronously move all bytes from [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
/// to [FileOutputStream](https://docs.gtk.org/gio/class.FileOutputStream.html)
/// * require `IOStream` reference to keep `Connection` active in async thread
pub fn from_stream_async(
    io_stream: impl IsA<IOStream>,
    file_output_stream: FileOutputStream,
    cancellable: Cancellable,
    priority: Priority,
    (chunk, limit, mut total): (
        usize,         // bytes_in_chunk
        Option<usize>, // bytes_total_limit, `None` to unlimited
        usize,         // bytes_total
    ),
    (on_chunk, on_complete): (
        impl Fn(Bytes, usize) + 'static, // on_chunk
        impl FnOnce(Result<(FileOutputStream, usize), Error>) + 'static, // on_complete
    ),
) {
    io_stream.input_stream().read_bytes_async(
        chunk,
        priority,
        Some(&cancellable.clone()),
        move |result| match result {
            Ok(bytes) => {
                total += bytes.len();
                on_chunk(bytes.clone(), total);

                if let Some(limit) = limit {
                    if total > limit {
                        return on_complete(Err(Error::BytesTotal(total, limit)));
                    }
                }

                if bytes.len() == 0 {
                    return on_complete(Ok((file_output_stream, total)));
                }

                file_output_stream.clone().write_async(
                    bytes.clone(),
                    priority,
                    Some(&cancellable.clone()),
                    move |result| {
                        match result {
                            Ok(_) => {
                                // continue read..
                                from_stream_async(
                                    io_stream,
                                    file_output_stream,
                                    cancellable,
                                    priority,
                                    (chunk, limit, total),
                                    (on_chunk, on_complete),
                                );
                            }
                            Err((bytes, e)) => {
                                on_complete(Err(Error::OutputStream(bytes.clone(), e)))
                            }
                        }
                    },
                );
            }
            Err(e) => on_complete(Err(Error::InputStream(e))),
        },
    )
}
