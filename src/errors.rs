use std::fmt;
use std::error;
use std::io;

use base64::Base64Error;

#[derive(Debug)]
pub enum WeChatError {
    InvalidSignature,
    InvalidAppId,
    InvalidBase64(Base64Error),
    ClientError { errcode: i32, errmsg: String },
    IOError(io::Error),
}

impl fmt::Display for WeChatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WeChatError::InvalidSignature => write!(f, "Invalid signature"),
            WeChatError::InvalidAppId => write!(f, "Invalid app_id"),
            WeChatError::InvalidBase64(ref err) => err.fmt(f),
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
            WeChatError::InvalidBase64(ref err) => err.description(),
            WeChatError::ClientError { ref errmsg, .. } => errmsg,
            WeChatError::IOError(ref err) => err.description(),
        }
    }
}

impl From<Base64Error> for WeChatError {
    fn from(err: Base64Error) -> WeChatError {
        WeChatError::InvalidBase64(err)
    }
}

impl From<io::Error> for WeChatError {
    fn from(err: io::Error) -> WeChatError {
        WeChatError::IOError(err)
    }
}
