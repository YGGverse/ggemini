use gio::*;
use glib::*;

use ggemini::client::connection::Request;

#[test]
fn client_connection_request_gemini() {
    const REQUEST: &str = "gemini://geminiprotocol.net/";
    assert_eq!(
        Request::Gemini(ggemini::client::connection::Gemini {
            uri: Uri::parse(REQUEST, UriFlags::NONE).unwrap()
        })
        .header(),
        format!("{REQUEST}\r\n")
    );
}

#[test]
fn client_connection_request_titan() {
    const DATA: &[u8] = &[1, 2, 3];
    const MIME: &str = "plain/text";
    const TOKEN: &str = "token";
    assert_eq!(
        Request::Titan(ggemini::client::connection::Titan {
            uri: Uri::parse(
                "titan://geminiprotocol.net/raw/Test?key=value",
                UriFlags::NONE
            )
            .unwrap(),
            data: Bytes::from(DATA),
            mime: Some(MIME.to_string()),
            token: Some(TOKEN.to_string())
        })
        .header(),
        format!(
            "titan://geminiprotocol.net/raw/Test;size={};mime={MIME};token={TOKEN}?key=value\r\n",
            DATA.len(),
        )
    );
}

// @TODO
