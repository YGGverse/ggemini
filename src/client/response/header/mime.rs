pub mod error;
pub use error::Error;

use glib::{Bytes, GString, Uri};
use std::path::Path;

/// https://geminiprotocol.net/docs/gemtext-specification.gmi#media-type-parameters
pub enum Mime {
    TextGemini,
    TextPlain,
    ImagePng,
    ImageGif,
    ImageJpeg,
    ImageWebp,
} // @TODO

impl Mime {
    pub fn from_header(bytes: &Bytes) -> Result<Self, Error> {
        match bytes.get(..) {
            Some(bytes) => match GString::from_utf8(bytes.to_vec()) {
                Ok(string) => Self::from_string(string.as_str()),
                Err(_) => Err(Error::Decode),
            },
            None => Err(Error::Undefined),
        }
    }

    pub fn from_path(path: &Path) -> Result<Self, Error> {
        match path.extension().and_then(|extension| extension.to_str()) {
            Some("gmi" | "gemini") => Ok(Self::TextGemini),
            Some("txt") => Ok(Self::TextPlain),
            Some("png") => Ok(Self::ImagePng),
            Some("gif") => Ok(Self::ImageGif),
            Some("jpeg" | "jpg") => Ok(Self::ImageJpeg),
            Some("webp") => Ok(Self::ImageWebp),
            _ => Err(Error::Undefined),
        }
    }

    pub fn from_string(value: &str) -> Result<Self, Error> {
        if value.contains("text/gemini") {
            return Ok(Self::TextGemini);
        }

        if value.contains("text/plain") {
            return Ok(Self::TextPlain);
        }

        if value.contains("image/gif") {
            return Ok(Self::ImageGif);
        }

        if value.contains("image/jpeg") {
            return Ok(Self::ImageJpeg);
        }

        if value.contains("image/webp") {
            return Ok(Self::ImageWebp);
        }

        if value.contains("image/png") {
            return Ok(Self::ImagePng);
        }

        Err(Error::Undefined)
    }

    pub fn from_uri(uri: &Uri) -> Result<Self, Error> {
        Self::from_path(Path::new(&uri.to_string()))
    }
}
