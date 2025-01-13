use gio::*;
use glib::*;

use ggemini::client::connection::Request;

#[test]
fn client_connection_request_gemini() {
    const REQUEST: &str = "gemini://geminiprotocol.net/";
    assert_eq!(
        &match Request::gemini(Uri::parse(REQUEST, UriFlags::NONE).unwrap()) {
            Request::Gemini(request) => request.uri.to_string(),
            Request::Titan(_) => panic!(),
        },
        REQUEST
    );
}

// @TODO
