//! Gemini response could have different MIME type for data.
//! Use one of these components to parse response according to content type expected.
//!
//! * MIME type could be detected using `client::response::Meta` parser

pub mod text;
