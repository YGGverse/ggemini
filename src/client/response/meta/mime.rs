//! MIME type parser for different data types:
//!
//! * UTF-8 buffer with entire response or just with meta slice (that include entire **header**)
//! * String (that include **header**)
//! * [Uri](https://docs.gtk.org/glib/struct.Uri.html) (that include **extension**)
//! * `std::Path` (that include **extension**)

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
    ImageSvg,
    ImageWebp,
    // Audio
    AudioFlac,
    AudioMpeg,
    AudioOgg,
} // @TODO

impl Mime {
    /// Create new `Self` from UTF-8 buffer (that includes **header**)
    ///
    /// * result could be `None` for some [status codes](https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes)
    /// that does not expect MIME type in header
    /// * includes `Self::from_string` parser,
    /// it means that given buffer should contain some **header** (not filepath or any other type of strings)
    pub fn from_utf8(buffer: &[u8]) -> Result<Option<Self>, Error> {
        // Define max buffer length for this method
        const MAX_LEN: usize = 0x400; // 1024

        // Calculate buffer length once
        let len = buffer.len();

        // Parse meta bytes only
        match buffer.get(..if len > MAX_LEN { MAX_LEN } else { len }) {
            Some(value) => match GString::from_utf8(value.into()) {
                Ok(string) => Self::from_string(string.as_str()),
                Err(_) => Err(Error::Decode),
            },
            None => Err(Error::Protocol),
        }
    }

    /// Create new `Self` from `std::Path` that includes file **extension**
    pub fn from_path(path: &Path) -> Result<Self, Error> {
        match path.extension().and_then(|extension| extension.to_str()) {
            // Text
            Some("gmi" | "gemini") => Ok(Self::TextGemini),
            Some("txt") => Ok(Self::TextPlain),

            // Image
            Some("gif") => Ok(Self::ImageGif),
            Some("jpeg" | "jpg") => Ok(Self::ImageJpeg),
            Some("png") => Ok(Self::ImagePng),
            Some("svg") => Ok(Self::ImageSvg),
            Some("webp") => Ok(Self::ImageWebp),

            // Audio
            Some("flac") => Ok(Self::AudioFlac),
            Some("mp3") => Ok(Self::AudioMpeg),
            Some("oga" | "ogg" | "opus" | "spx") => Ok(Self::AudioOgg),
            _ => Err(Error::Undefined),
        } // @TODO extension to lowercase
    }

    /// Create new `Self` from string that includes **header**
    ///
    /// **Return**
    ///
    /// * `None` if MIME type not found
    /// * `Error::Undefined` if status code 2* and type not found in `Mime` enum
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

        if value.contains("image/png") {
            return Ok(Some(Self::ImagePng));
        }

        if value.contains("image/svg+xml") {
            return Ok(Some(Self::ImageSvg));
        }

        if value.contains("image/webp") {
            return Ok(Some(Self::ImageWebp));
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

        // Some type exist, but not defined yet (on status code is 2*)
        if value.starts_with("2") && value.contains("/") {
            return Err(Error::Undefined);
        }

        // Done
        Ok(None) // may be empty (status code ^2*)
    }

    /// Create new `Self` from [Uri](https://docs.gtk.org/glib/struct.Uri.html)
    /// that includes file **extension**
    pub fn from_uri(uri: &Uri) -> Result<Self, Error> {
        Self::from_path(Path::new(&uri.to_string()))
    }
}
