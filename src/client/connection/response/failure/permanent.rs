pub mod bad_request;
pub mod default;
pub mod error;
pub mod gone;
pub mod not_found;
pub mod proxy_request_refused;

pub use bad_request::BadRequest;
pub use default::Default;
pub use error::Error;
pub use gone::Gone;
pub use not_found::NotFound;
pub use proxy_request_refused::ProxyRequestRefused;

const CODE: u8 = b'5';

/// https://geminiprotocol.net/docs/protocol-specification.gmi#permanent-failure
pub enum Permanent {
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-50
    Default(Default),
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-51-not-found
    NotFound(NotFound),
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-52-gone
    Gone(Gone),
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-53-proxy-request-refused
    ProxyRequestRefused(ProxyRequestRefused),
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-59-bad-request
    BadRequest(BadRequest),
}

impl Permanent {
    // Constructors

    /// Create new `Self` from buffer include header bytes
    pub fn from_utf8(buffer: &[u8]) -> Result<Self, Error> {
        match buffer.first() {
            Some(b) => match *b {
                CODE => match buffer.get(1) {
                    Some(b) => match *b {
                        b'0' => Ok(Self::Default(
                            Default::from_utf8(buffer).map_err(Error::Default)?,
                        )),
                        b'1' => Ok(Self::NotFound(
                            NotFound::from_utf8(buffer).map_err(Error::NotFound)?,
                        )),
                        b'2' => Ok(Self::Gone(Gone::from_utf8(buffer).map_err(Error::Gone)?)),
                        b'3' => Ok(Self::ProxyRequestRefused(
                            ProxyRequestRefused::from_utf8(buffer)
                                .map_err(Error::ProxyRequestRefused)?,
                        )),
                        b'9' => Ok(Self::BadRequest(
                            BadRequest::from_utf8(buffer).map_err(Error::BadRequest)?,
                        )),
                        b => Err(Error::SecondByte(b)),
                    },
                    None => Err(Error::UndefinedSecondByte),
                },
                b => Err(Error::FirstByte(b)),
            },
            None => Err(Error::UndefinedFirstByte),
        }
    }

    // Getters

    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Default(default) => default.message(),
            Self::NotFound(not_found) => not_found.message(),
            Self::Gone(gone) => gone.message(),
            Self::ProxyRequestRefused(proxy_request_refused) => proxy_request_refused.message(),
            Self::BadRequest(bad_request) => bad_request.message(),
        }
    }

    /// Get optional message for `Self`
    /// * if the optional message not provided by the server, return children `DEFAULT_MESSAGE`
    pub fn message_or_default(&self) -> &str {
        match self {
            Self::Default(default) => default.message_or_default(),
            Self::NotFound(not_found) => not_found.message_or_default(),
            Self::Gone(gone) => gone.message_or_default(),
            Self::ProxyRequestRefused(proxy_request_refused) => {
                proxy_request_refused.message_or_default()
            }
            Self::BadRequest(bad_request) => bad_request.message_or_default(),
        }
    }

    /// Get header string of `Self`
    pub fn as_str(&self) -> &str {
        match self {
            Self::Default(default) => default.as_str(),
            Self::NotFound(not_found) => not_found.as_str(),
            Self::Gone(gone) => gone.as_str(),
            Self::ProxyRequestRefused(proxy_request_refused) => proxy_request_refused.as_str(),
            Self::BadRequest(bad_request) => bad_request.as_str(),
        }
    }

    /// Get header bytes of `Self`
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Default(default) => default.as_bytes(),
            Self::NotFound(not_found) => not_found.as_bytes(),
            Self::Gone(gone) => gone.as_bytes(),
            Self::ProxyRequestRefused(proxy_request_refused) => proxy_request_refused.as_bytes(),
            Self::BadRequest(bad_request) => bad_request.as_bytes(),
        }
    }
}

#[test]
fn test() {
    fn t(source: &str, message: Option<&str>) {
        let b = source.as_bytes();
        let i = Permanent::from_utf8(b).unwrap();
        assert_eq!(i.message(), message);
        assert_eq!(i.as_str(), source);
        assert_eq!(i.as_bytes(), b);
    }
    // 50
    t("50 Message\r\n", Some("Message"));
    t("50\r\n", None);
    // 51
    t("51 Message\r\n", Some("Message"));
    t("51\r\n", None);
    // 52
    t("52 Message\r\n", Some("Message"));
    t("52\r\n", None);
    // 53
    t("53 Message\r\n", Some("Message"));
    t("53\r\n", None);
    // 59
    t("59 Message\r\n", Some("Message"));
    t("59\r\n", None);
}
