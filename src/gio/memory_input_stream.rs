pub mod error;
pub use error::Error;

use gio::{
    prelude::{IOStreamExt, InputStreamExt, MemoryInputStreamExt},
    Cancellable, IOStream, MemoryInputStream,
};
use glib::{object::IsA, Priority};

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
    (chunk, limit): (usize, usize),
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
        (chunk, limit, 0),
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
    (chunk, limit, mut total): (usize, usize, usize),
    (on_chunk, on_complete): (
        impl Fn(usize, usize) + 'static,
        impl FnOnce(Result<(MemoryInputStream, usize), Error>) + 'static,
    ),
) {
    io_stream.input_stream().read_bytes_async(
        chunk,
        priority,
        Some(&cancellable.clone()),
        move |result| match result {
            Ok(bytes) => {
                let len = bytes.len(); // calculate once

                total += len;
                on_chunk(len, total);

                memory_input_stream.add_bytes(&bytes);

                // prevent memory overflow on size `limit` reached
                // * add last received bytes into the `memory_input_stream` anyway (to prevent data lost),
                //   it's safe because limited to the `chunk` size
                if total > limit {
                    return on_complete(Err(Error::BytesTotal(memory_input_stream, total, limit)));
                }

                // is the next iteration required?
                if len < chunk // some servers may close the connection after first chunk request (@TODO this condition wants review)
                || len == 0
                {
                    return on_complete(Ok((memory_input_stream, total)));
                }

                // continue reading..
                for_memory_input_stream_async(
                    memory_input_stream,
                    io_stream,
                    priority,
                    cancellable,
                    (chunk, limit, total),
                    (on_chunk, on_complete),
                )
            }
            Err(e) => on_complete(Err(Error::InputStream(memory_input_stream, e))),
        },
    )
}
