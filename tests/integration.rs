use gio::*;
use glib::*;

use ggemini::client::connection::request::Gemini;

#[test]
fn client_connection_request_gemini_build() {
    const REQUEST: &str = "gemini://geminiprotocol.net/";

    let request = Gemini::build(Uri::parse(REQUEST, UriFlags::NONE).unwrap());

    assert_eq!(&request.uri.to_string(), REQUEST);
}

// @TODO
