pub mod error;
pub mod response;
pub mod socket;

pub use error::Error;
pub use response::Response;
pub use socket::Socket;

use glib::Uri;

/// High-level API to make single async request
///
/// 1. open new socket connection for [Uri](https://docs.gtk.org/glib/struct.Uri.html)
/// 2. send request
/// 3. read response
/// 4. close connection
/// 5. return `Result<Response, Error>` to `callback` function
pub fn simple_socket_request_async(
    uri: Uri,
    callback: impl FnOnce(Result<Response, Error>) + 'static,
) {
    Socket::new().connect_async(uri.clone(), None, move |connect| match connect {
        Ok(connection) => {
            connection.request_async(uri, None, None, None, move |connection, response| {
                connection.close_async(
                    None,
                    None,
                    Some(|close| {
                        callback(match close {
                            Ok(_) => match response {
                                Ok(buffer) => match Response::from_utf8(&buffer) {
                                    Ok(response) => Ok(response),
                                    Err(_) => Err(Error::Response),
                                },
                                Err(_) => Err(Error::Request),
                            },
                            Err(_) => Err(Error::Close),
                        })
                    }),
                );
            })
        }
        Err(_) => callback(Err(Error::Connection)),
    });
}
