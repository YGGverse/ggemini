pub mod error;
pub mod response;
pub mod socket;

pub use error::Error;

pub use response::Response;
pub use socket::Socket;

use gio::Cancellable;
use glib::{Priority, Uri};

/// High-level API to make async request, get `Response` and close connection.
pub fn request_async(
    uri: Uri,
    cancelable: Option<Cancellable>,
    priority: Option<Priority>,
    callback: impl FnOnce(Result<Response, Error>) + 'static,
) {
    // Create new socket connection
    Socket::new().connect_async(
        uri.clone(),
        cancelable.clone(),
        move |connect| match connect {
            Ok(connection) => {
                connection.request_async(uri, cancelable, priority, None, |request| {
                    callback(match request {
                        Ok(buffer) => match Response::from_utf8(&buffer) {
                            Ok(response) => Ok(response),
                            Err(_) => Err(Error::Response),
                        },
                        Err(_) => Err(Error::Request),
                    });

                    //connection.close_async(cancelable, priority, |_| {}); // @TODO
                })
            }
            Err(_) => callback(Err(Error::Connection)),
        },
    );
}
