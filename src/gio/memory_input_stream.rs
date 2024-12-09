pub mod error;
pub use error::Error;

use gio::{
    prelude::{IOStreamExt, InputStreamExt, MemoryInputStreamExt},
    Cancellable, IOStream, MemoryInputStream,
};
use glib::{object::IsA, Bytes, Priority};

/// Asynchronously create new [MemoryInputStream](https://docs.gtk.org/gio/class.MemoryInputStream.html)
/// from [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
///
/// **Useful for**
/// * safe read (of memory overflow) to dynamically allocated buffer, where final size of target data unknown
/// * calculate bytes processed on chunk load
pub fn from_stream_async(
    base_io_stream: impl IsA<IOStream>,
    cancelable: Cancellable,
    priority: Priority,
    bytes_in_chunk: usize,
    bytes_total_limit: usize,
    on_chunk: impl Fn(Bytes, usize) + 'static,
    on_complete: impl FnOnce(Result<MemoryInputStream, Error>) + 'static,
) {
    move_all_from_stream_async(
        base_io_stream,
        MemoryInputStream::new(),
        cancelable,
        priority,
        (bytes_in_chunk, bytes_total_limit, 0),
        (on_chunk, on_complete),
    );
}

/// Asynchronously move all bytes from [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
/// to [MemoryInputStream](https://docs.gtk.org/gio/class.MemoryInputStream.html)
/// * require `IOStream` reference to keep `Connection` active in async thread
pub fn move_all_from_stream_async(
    base_io_stream: impl IsA<IOStream>,
    memory_input_stream: MemoryInputStream,
    cancellable: Cancellable,
    priority: Priority,
    bytes: (
        usize, // bytes_in_chunk
        usize, // bytes_total_limit
        usize, // bytes_total
    ),
    callback: (
        impl Fn(Bytes, usize) + 'static,                         // on_chunk
        impl FnOnce(Result<MemoryInputStream, Error>) + 'static, // on_complete
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
                if bytes_total > bytes_total_limit {
                    return on_complete(Err(Error::BytesTotal(bytes_total, bytes_total_limit)));
                }

                // No bytes were read, end of stream
                if bytes.len() == 0 {
                    return on_complete(Ok(memory_input_stream));
                }

                // Write chunk bytes
                memory_input_stream.add_bytes(&bytes);

                // Continue
                move_all_from_stream_async(
                    base_io_stream,
                    memory_input_stream,
                    cancellable,
                    priority,
                    (bytes_in_chunk, bytes_total_limit, bytes_total),
                    (on_chunk, on_complete),
                );
            }
            Err(e) => {
                on_complete(Err(Error::InputStream(e)));
            }
        },
    );
}
