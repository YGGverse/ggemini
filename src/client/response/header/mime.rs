pub mod error;
pub use error::Error;

use glib::{GString, Uri};
use std::path::Path;

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
    pub fn from_header(buffer: &[u8]) -> Result<Self, Error> {
        match buffer.get(..) {
            Some(value) => match GString::from_utf8(value.to_vec()) {
                Ok(string) => Self::from_string(string.as_str()),
                Err(_) => Err(Error::Decode),
            },
            None => Err(Error::Undefined),
        }
    }

    pub fn from_path(path: &Path) -> Result<Self, Error> {
        match path.extension().and_then(|extension| extension.to_str()) {
            // Text
            Some("gmi" | "gemini") => Ok(Self::TextGemini),
            // Image
            Some("txt") => Ok(Self::TextPlain),
            Some("png") => Ok(Self::ImagePng),
            Some("gif") => Ok(Self::ImageGif),
            Some("jpeg" | "jpg") => Ok(Self::ImageJpeg),
            Some("webp") => Ok(Self::ImageWebp),
            // Audio
            Some("flac") => Ok(Self::AudioFlac),
            Some("mp3") => Ok(Self::AudioMpeg),
            Some("ogg" | "opus" | "oga" | "spx") => Ok(Self::AudioOgg),
            _ => Err(Error::Undefined),
        } // @TODO extension to lowercase
    }

    pub fn from_string(value: &str) -> Result<Self, Error> {
        // Text
        if value.contains("text/gemini") {
            return Ok(Self::TextGemini);
        }

        if value.contains("text/plain") {
            return Ok(Self::TextPlain);
        }

        // Image
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

        // Audio
        if value.contains("audio/flac") {
            return Ok(Self::AudioFlac);
        }

        if value.contains("audio/mpeg") {
            return Ok(Self::AudioMpeg);
        }

        if value.contains("audio/ogg") {
            return Ok(Self::AudioOgg);
        }

        Err(Error::Undefined)
    }

    pub fn from_uri(uri: &Uri) -> Result<Self, Error> {
        Self::from_path(Path::new(&uri.to_string()))
    }
}
