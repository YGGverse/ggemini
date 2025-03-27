pub mod error;
pub mod size;

pub use error::Error;
pub use size::Size;

use gio::{
    Cancellable, IOStream, MemoryInputStream,
    prelude::{IOStreamExt, InputStreamExt, MemoryInputStreamExt},
};
use glib::{Priority, object::IsA};

/// Asynchronously create new [MemoryInputStream](https://docs.gtk.org/gio/class.MemoryInputStream.html)
/// from [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
///
/// **Useful for**
/// * safe read (of memory overflow) to dynamically allocated buffer, where final size of target data unknown
/// * calculate bytes processed on chunk load
pub fn from_stream_async(
    io_stream: impl IsA<IOStream>,
    priority: Priority,
    cancelable: Cancellable,
    size: Size,
    (on_chunk, on_complete): (
        impl Fn(usize, usize) + 'static,
        impl FnOnce(Result<(MemoryInputStream, usize), Error>) + 'static,
    ),
) {
    for_memory_input_stream_async(
        MemoryInputStream::new(),
        io_stream,
        priority,
        cancelable,
        size,
        (on_chunk, on_complete),
    );
}

/// Asynchronously move all bytes from [IOStream](https://docs.gtk.org/gio/class.IOStream.html)
/// to [MemoryInputStream](https://docs.gtk.org/gio/class.MemoryInputStream.html)
/// * require `IOStream` reference to keep `Connection` active in async thread
pub fn for_memory_input_stream_async(
    memory_input_stream: MemoryInputStream,
    io_stream: impl IsA<IOStream>,
    priority: Priority,
    cancellable: Cancellable,
    mut size: Size,
    (on_chunk, on_complete): (
        impl Fn(usize, usize) + 'static,
        impl FnOnce(Result<(MemoryInputStream, usize), Error>) + 'static,
    ),
) {
    io_stream.input_stream().read_bytes_async(
        size.chunk,
        priority,
        Some(&cancellable.clone()),
        move |result| match result {
            Ok(bytes) => {
                let len = bytes.len(); // calculate once

                // is end of stream
                if len == 0 {
                    return on_complete(Ok((memory_input_stream, size.total)));
                }

                // callback chunk function
                size.total += len;
                on_chunk(len, size.total);

                // push bytes into the memory pool
                memory_input_stream.add_bytes(&bytes);

                // prevent memory overflow
                if size.total > size.limit {
                    return on_complete(Err(Error::BytesTotal(
                        memory_input_stream,
                        size.total,
                        size.limit,
                    )));
                }

                // handle next chunk..
                for_memory_input_stream_async(
                    memory_input_stream,
                    io_stream,
                    priority,
                    cancellable,
                    size,
                    (on_chunk, on_complete),
                )
            }
            Err(e) => on_complete(Err(Error::InputStream(memory_input_stream, e))),
        },
    )
}
