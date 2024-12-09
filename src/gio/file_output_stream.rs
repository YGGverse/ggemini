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
pub fn move_all_from_stream_async(
    base_io_stream: impl IsA<IOStream>,
    file_output_stream: FileOutputStream,
    cancellable: Cancellable,
    priority: Priority,
    bytes: (
        usize,         // bytes_in_chunk
        Option<usize>, // bytes_total_limit, `None` to unlimited
        usize,         // bytes_total
    ),
    callback: (
        impl Fn(Bytes, usize) + 'static,                        // on_chunk
        impl FnOnce(Result<FileOutputStream, Error>) + 'static, // on_complete
    ),
) {
    let (on_chunk, on_complete) = callback;
    let (bytes_in_chunk, bytes_total_limit, bytes_total) = bytes;

    base_io_stream.input_stream().read_bytes_async(
        bytes_in_chunk,
        priority,
        Some(&cancellable.clone()),
        move |result| match result {
            Ok(bytes) => {
                // Update bytes total
                let bytes_total = bytes_total + bytes.len();

                // Callback chunk function
                on_chunk(bytes.clone(), bytes_total);

                // Validate max size
                if let Some(bytes_total_limit) = bytes_total_limit {
                    if bytes_total > bytes_total_limit {
                        return on_complete(Err(Error::BytesTotal(bytes_total, bytes_total_limit)));
                    }
                }

                // No bytes were read, end of stream
                if bytes.len() == 0 {
                    return on_complete(Ok(file_output_stream));
                }

                // Write chunk bytes
                file_output_stream.clone().write_async(
                    bytes.clone(),
                    priority,
                    Some(&cancellable.clone()),
                    move |result| {
                        match result {
                            Ok(_) => {
                                // Continue
                                move_all_from_stream_async(
                                    base_io_stream,
                                    file_output_stream,
                                    cancellable,
                                    priority,
                                    (bytes_in_chunk, bytes_total_limit, bytes_total),
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
            Err(e) => {
                on_complete(Err(Error::InputStream(e)));
            }
        },
    );
}
