use glib::{GString, Uri};
use std::path::Path;

pub enum Mime {
    TextGemini,
    TextPlain,
    ImagePng,
    ImageGif,
    ImageJpeg,
    ImageWebp,
} // @TODO

pub fn from_header(buffer: &[u8] /* @TODO */) -> Option<Mime> {
    from_string(&match GString::from_utf8(buffer.to_vec()) {
        Ok(result) => result,
        Err(_) => return None, // @TODO error handler?
    })
}

pub fn from_path(path: &Path) -> Option<Mime> {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("gmi") | Some("gemini") => Some(Mime::TextGemini),
        Some("txt") => Some(Mime::TextPlain),
        Some("png") => Some(Mime::ImagePng),
        Some("gif") => Some(Mime::ImageGif),
        Some("jpeg") | Some("jpg") => Some(Mime::ImageJpeg),
        Some("webp") => Some(Mime::ImageWebp),
        _ => None,
    }
}

pub fn from_string(value: &str) -> Option<Mime> {
    if value.contains("text/gemini") {
        return Some(Mime::TextGemini);
    }

    if value.contains("text/plain") {
        return Some(Mime::TextPlain);
    }

    if value.contains("image/gif") {
        return Some(Mime::ImageGif);
    }

    if value.contains("image/jpeg") {
        return Some(Mime::ImageJpeg);
    }

    if value.contains("image/webp") {
        return Some(Mime::ImageWebp);
    }

    if value.contains("image/png") {
        return Some(Mime::ImagePng);
    }

    None
}

pub fn from_uri(uri: &Uri) -> Option<Mime> {
    from_path(Path::new(&uri.to_string()))
}
