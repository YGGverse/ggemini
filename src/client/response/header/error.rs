#[derive(Debug)]
pub enum Error {
    Buffer,
    InputStream,
    Protocol,
    StatusDecode,
    StatusUndefined,
}
