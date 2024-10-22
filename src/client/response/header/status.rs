pub mod error;
pub use error::Error;

use glib::GString;

/// https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes
pub enum Status {
    Input,
    SensitiveInput,
    Success,
    Redirect,
} // @TODO

pub fn from_header(buffer: &[u8] /* @TODO */) -> Result<Status, Error> {
    match buffer.get(0..2) {
        Some(bytes) => match GString::from_utf8(bytes.to_vec()) {
            Ok(string) => from_string(string.as_str()),
            Err(_) => Err(Error::Decode),
        },
        None => Err(Error::Undefined),
    }
}

pub fn from_string(code: &str) -> Result<Status, Error> {
    match code {
        "10" => Ok(Status::Input),
        "11" => Ok(Status::SensitiveInput),
        "20" => Ok(Status::Success),
        _ => Err(Error::Undefined),
    }
}
