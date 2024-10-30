#[derive(Debug)]
pub enum Error {
    DataDecode,
    DataProtocol,
    InputStream,
    MimeDecode,
    MimeProtocol,
    MimeUndefined,
    Protocol,
    StatusDecode,
    StatusProtocol,
    StatusUndefined,
}
