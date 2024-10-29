pub mod error;
pub use error::Error;

use gio::{
    prelude::{IOStreamExt, InputStreamExt, MemoryInputStreamExt},
    Cancellable, MemoryInputStream, SocketConnection,
};
use glib::{Bytes, Priority};

/// Asynchronously create new [MemoryInputStream](https://docs.gtk.org/gio/class.MemoryInputStream.html)
/// from [InputStream](https://docs.gtk.org/gio/class.InputStream.html)
/// for given [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html)
///
/// Useful to create dynamically allocated, memory-safe buffer
/// from remote connections, where final size of target data could not be known by Gemini protocol restrictions.
/// Also, could be useful for [Pixbuf](https://docs.gtk.org/gdk-pixbuf/class.Pixbuf.html) or
/// loading widgets like [Spinner](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.Spinner.html)
/// to display bytes on async data loading.
///
/// * this function takes entire `SocketConnection` reference (not `MemoryInputStream`) just to keep connection alive in the async context
///
/// **Implementation**
///
/// Implements low-level `read_all_from_socket_connection_async` function:
/// * recursively read all bytes from `InputStream` for `SocketConnection` according to `bytes_in_chunk` argument
/// * calculates total bytes length on every chunk iteration, validate sum with `bytes_total_limit` argument
/// * stop reading `InputStream` with `Result` on zero bytes in chunk received
/// * applies optional callback functions:
///   * `on_chunk` - return reference to [Bytes](https://docs.gtk.org/glib/struct.Bytes.html) and `bytes_total` collected for every chunk in reading loop
///   * `on_complete` - return `MemoryInputStream` on success or `Error` on failure as `Result`
pub fn from_socket_connection_async(
    socket_connection: SocketConnection,
    cancelable: Option<Cancellable>,
    priority: Priority,
    bytes_in_chunk: usize,
    bytes_total_limit: usize,
    on_chunk: Option<impl Fn((&Bytes, &usize)) + 'static>,
    on_complete: Option<impl FnOnce(Result<MemoryInputStream, (Error, Option<&str>)>) + 'static>,
) {
    read_all_from_socket_connection_async(
        MemoryInputStream::new(),
        socket_connection,
        cancelable,
        priority,
        bytes_in_chunk,
        bytes_total_limit,
        0, // initial `bytes_total` value
        on_chunk,
        on_complete,
    );
}

/// Low-level helper for `from_socket_connection_async` function,
/// also provides public API for external usage.
///
/// Asynchronously read [InputStream](https://docs.gtk.org/gio/class.InputStream.html)
/// from [SocketConnection](https://docs.gtk.org/gio/class.SocketConnection.html)
/// to given [MemoryInputStream](https://docs.gtk.org/gio/class.MemoryInputStream.html).
/// Applies optional `on_chunk` and `on_complete` callback functions.
///
/// Useful to create dynamically allocated, memory-safe buffer
/// from remote connections, where final size of target data could not be known by Gemini protocol restrictions.
/// Also, could be useful for [Pixbuf](https://docs.gtk.org/gdk-pixbuf/class.Pixbuf.html) or
/// loading widgets like [Spinner](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.Spinner.html)
/// to display bytes on async data loading.
///
/// * this function takes entire `SocketConnection` reference (not `MemoryInputStream`) just to keep connection alive in the async context
///
/// **Implementation**
///
/// * recursively read all bytes from `InputStream` for `SocketConnection` according to `bytes_in_chunk` argument
/// * calculates total bytes length on every chunk iteration, validate sum with `bytes_total_limit` argument
/// * stop reading `InputStream` with `Result` on zero bytes in chunk received, otherwise continue next chunk request in loop
/// * applies optional callback functions:
///   * `on_chunk` - return reference to [Bytes](https://docs.gtk.org/glib/struct.Bytes.html) and `bytes_total` collected for every chunk in reading loop
///   * `on_complete` - return `MemoryInputStream` on success or `Error` on failure as `Result`
pub fn read_all_from_socket_connection_async(
    memory_input_stream: MemoryInputStream,
    socket_connection: SocketConnection,
    cancelable: Option<Cancellable>,
    priority: Priority,
    bytes_in_chunk: usize,
    bytes_total_limit: usize,
    bytes_total: usize,
    on_chunk: Option<impl Fn((&Bytes, &usize)) + 'static>,
    on_complete: Option<impl FnOnce(Result<MemoryInputStream, (Error, Option<&str>)>) + 'static>,
) {
    socket_connection.input_stream().read_bytes_async(
        bytes_in_chunk,
        priority,
        cancelable.clone().as_ref(),
        move |result| match result {
            Ok(bytes) => {
                // Update bytes total
                let bytes_total = bytes_total + bytes.len();

                // Callback chunk function
                if let Some(ref callback) = on_chunk {
                    callback((&bytes, &bytes_total));
                }

                // Validate max size
                if bytes_total > bytes_total_limit {
                    if let Some(callback) = on_complete {
                        callback(Err((Error::BytesTotal, None)));
                    }
                    return; // break
                }

                // No bytes were read, end of stream
                if bytes.len() == 0 {
                    if let Some(callback) = on_complete {
                        callback(Ok(memory_input_stream));
                    }
                    return; // break
                }

                // Write chunk bytes
                memory_input_stream.add_bytes(&bytes);

                // Continue
                read_all_from_socket_connection_async(
                    memory_input_stream,
                    socket_connection,
                    cancelable,
                    priority,
                    bytes_in_chunk,
                    bytes_total_limit,
                    bytes_total,
                    on_chunk,
                    on_complete,
                );
            }
            Err(reason) => {
                if let Some(callback) = on_complete {
                    callback(Err((Error::InputStream, Some(reason.message()))));
                }
            }
        },
    );
}
