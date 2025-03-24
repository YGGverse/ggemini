/// Request modes
pub enum Mode {
    /// Request header bytes only, process content bytes manually
    /// * useful for manual content type handle: text, stream or large content loaded by chunks
    HeaderOnly,
}
