use std::fmt;
use std::error;
use std::io;

#[derive(Debug)]
pub enum WeChatError {
    InvalidSignature,
    InvalidAppId,
    ClientError { errcode: i32, errmsg: String },
    IOError(io::Error),
}

impl fmt::Display for WeChatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WeChatError::InvalidSignature => write!(f, "Invalid signature"),
            WeChatError::InvalidAppId => write!(f, "Invalid app_id"),
            WeChatError::ClientError { errcode, ref errmsg } => write!(f, "Client error code: {}, message: {}", errcode, errmsg),
            WeChatError::IOError(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for WeChatError {

    fn description(&self) -> &str {
        match *self {
            WeChatError::InvalidSignature => "Invalid signature",
            WeChatError::InvalidAppId => "Invalid app_id",
            // WeChatError::InvalidBase64(ref err) => err.description(),
            WeChatError::ClientError { ref errmsg, .. } => errmsg,
            WeChatError::IOError(ref err) => err.description(),
        }
    }
}

impl From<io::Error> for WeChatError {
    fn from(err: io::Error) -> WeChatError {
        WeChatError::IOError(err)
    }
}
