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
    cancelable: Option<Cancellable>,
    priority: Priority,
    bytes_in_chunk: usize,
    bytes_total_limit: usize,
    on_chunk: impl Fn((Bytes, usize)) + 'static,
    on_complete: impl FnOnce(Result<MemoryInputStream, (Error, Option<&str>)>) + 'static,
) {
    read_all_from_stream_async(
        MemoryInputStream::new(),
        base_io_stream,
        cancelable,
        priority,
        (bytes_in_chunk, bytes_total_limit, 0),
        (on_chunk, on_complete),
    );
}

/// Asynchronously read entire [InputStream](https://docs.gtk.org/gio/class.InputStream.html)
/// from [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
/// * require `IOStream` reference to keep `Connection` active in async thread
pub fn read_all_from_stream_async(
    memory_input_stream: MemoryInputStream,
    base_io_stream: impl IsA<IOStream>,
    cancelable: Option<Cancellable>,
    priority: Priority,
    bytes: (usize, usize, usize),
    callback: (
        impl Fn((Bytes, usize)) + 'static,
        impl FnOnce(Result<MemoryInputStream, (Error, Option<&str>)>) + 'static,
    ),
) {
    let (on_chunk, on_complete) = callback;
    let (bytes_in_chunk, bytes_total_limit, bytes_total) = bytes;

    base_io_stream.input_stream().read_bytes_async(
        bytes_in_chunk,
        priority,
        cancelable.clone().as_ref(),
        move |result| match result {
            Ok(bytes) => {
                // Update bytes total
                let bytes_total = bytes_total + bytes.len();

                // Callback chunk function
                on_chunk((bytes.clone(), bytes_total));

                // Validate max size
                if bytes_total > bytes_total_limit {
                    return on_complete(Err((Error::BytesTotal, None)));
                }

                // No bytes were read, end of stream
                if bytes.len() == 0 {
                    return on_complete(Ok(memory_input_stream));
                }

                // Write chunk bytes
                memory_input_stream.add_bytes(&bytes);

                // Continue
                read_all_from_stream_async(
                    memory_input_stream,
                    base_io_stream,
                    cancelable,
                    priority,
                    (bytes_in_chunk, bytes_total_limit, bytes_total),
                    (on_chunk, on_complete),
                );
            }
            Err(reason) => {
                on_complete(Err((Error::InputStream, Some(reason.message()))));
            }
        },
    );
}
