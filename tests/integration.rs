use gio::*;
use glib::*;

use ggemini::client::connection::Request;

#[test]
fn client_connection_request_gemini() {
    const REQUEST: &str = "gemini://geminiprotocol.net/";
    assert_eq!(
        &match Request::gemini(Uri::parse(REQUEST, UriFlags::NONE).unwrap()) {
            Request::Gemini(request) => request.uri.to_string(),
            _ => panic!(),
        },
        REQUEST
    );
}

#[test]
fn client_connection_request_titan() {
    const DATA: &[u8] = &[1, 2, 3];
    const MIME: &str = "plain/text";
    const TOKEN: &str = "token";
    const ARGUMENT: &str = "argument";
    const REQUEST: &str = "titan://geminiprotocol.net/raw/Test";
    assert_eq!(
        std::str::from_utf8(
            &Request::titan(
                Uri::parse(&format!("{REQUEST}?arg={ARGUMENT}"), UriFlags::NONE).unwrap(),
                DATA.to_vec(),
                Some(MIME.to_string()),
                Some(TOKEN.to_string()),
            )
            .to_bytes()
        )
        .unwrap(),
        format!(
            "{REQUEST};size={};mime={MIME};token={TOKEN}?arg={ARGUMENT}\r\n{}",
            DATA.len(),
            std::str::from_utf8(DATA).unwrap(),
        )
    );
}

// @TODO
