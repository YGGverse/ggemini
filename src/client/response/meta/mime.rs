pub mod error;
pub use error::Error;

use glib::{GString, Uri};
use std::path::Path;

pub const MAX_LEN: usize = 0x400; // 1024

/// https://geminiprotocol.net/docs/gemtext-specification.gmi#media-type-parameters
#[derive(Debug)]
pub enum Mime {
    // Text
    TextGemini,
    TextPlain,
    // Image
    ImageGif,
    ImageJpeg,
    ImagePng,
    ImageWebp,
    // Audio
    AudioFlac,
    AudioMpeg,
    AudioOgg,
} // @TODO

impl Mime {
    pub fn from_utf8(buffer: &[u8]) -> Result<Option<Self>, Error> {
        let len = buffer.len();
        match buffer.get(..if len > MAX_LEN { MAX_LEN } else { len }) {
            Some(value) => match GString::from_utf8(value.into()) {
                Ok(string) => Self::from_string(string.as_str()),
                Err(_) => Err(Error::Decode),
            },
            None => Err(Error::Protocol),
        }
    }

    pub fn from_path(path: &Path) -> Result<Self, Error> {
        match path.extension().and_then(|extension| extension.to_str()) {
            // Text
            Some("gmi" | "gemini") => Ok(Self::TextGemini),
            Some("txt") => Ok(Self::TextPlain),
            // Image
            Some("gif") => Ok(Self::ImageGif),
            Some("jpeg" | "jpg") => Ok(Self::ImageJpeg),
            Some("png") => Ok(Self::ImagePng),
            Some("webp") => Ok(Self::ImageWebp),
            // Audio
            Some("flac") => Ok(Self::AudioFlac),
            Some("mp3") => Ok(Self::AudioMpeg),
            Some("oga" | "ogg" | "opus" | "spx") => Ok(Self::AudioOgg),
            _ => Err(Error::Undefined),
        } // @TODO extension to lowercase
    }

    pub fn from_string(value: &str) -> Result<Option<Self>, Error> {
        // Text
        if value.contains("text/gemini") {
            return Ok(Some(Self::TextGemini));
        }

        if value.contains("text/plain") {
            return Ok(Some(Self::TextPlain));
        }

        // Image
        if value.contains("image/gif") {
            return Ok(Some(Self::ImageGif));
        }

        if value.contains("image/jpeg") {
            return Ok(Some(Self::ImageJpeg));
        }

        if value.contains("image/webp") {
            return Ok(Some(Self::ImageWebp));
        }

        if value.contains("image/png") {
            return Ok(Some(Self::ImagePng));
        }

        // Audio
        if value.contains("audio/flac") {
            return Ok(Some(Self::AudioFlac));
        }

        if value.contains("audio/mpeg") {
            return Ok(Some(Self::AudioMpeg));
        }

        if value.contains("audio/ogg") {
            return Ok(Some(Self::AudioOgg));
        }

        // Some type exist, but not defined yet
        if value.contains("/") {
            return Err(Error::Undefined);
        }

        // Done
        Ok(None) // may be empty (for some status codes)
    }

    pub fn from_uri(uri: &Uri) -> Result<Self, Error> {
        Self::from_path(Path::new(&uri.to_string()))
    }
}
