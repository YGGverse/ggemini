use glib::{Uri, UriFlags};

#[test]
fn simple_socket_request_async() {
    // Parse URI
    match Uri::parse("gemini://geminiprotocol.net/", UriFlags::NONE) {
        // Begin async request
        Ok(uri) => ggemini::client::simple_socket_request_async(uri, |response| match response {
            // Process response
            Ok(response) => {
                // Expect success status
                assert!(match response.header().status() {
                    Some(ggemini::client::response::header::Status::Success) => true,
                    _ => false,
                })
            }
            Err(_) => assert!(false),
        }),
        Err(_) => assert!(false),
    }
} // @TODO async
