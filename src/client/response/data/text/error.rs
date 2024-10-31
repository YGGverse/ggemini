#[derive(Debug)]
pub enum Error {
    BufferOverflow,
    Decode,
    InputStream,
}
