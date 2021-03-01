//! copyright © shaipe 2021 - present
//! 微信处理错误信息处理

use std::fmt;
use std::error;
use std::io;
use base64::DecodeError;

/// 微信处理错误
#[derive(Debug)]
pub enum WeChatError {
    
    InvalidSignature,
    InvalidAppId,
    InvalidBase64(DecodeError),
    ClientError { errcode: i32, errmsg: String },
    IOError(io::Error),
    InvalidValue,
    Custom { code: i32, msg: String },
}

/// 实现调试输出
impl fmt::Display for WeChatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WeChatError::InvalidSignature => write!(f, "Invalid signature"),
            WeChatError::InvalidAppId => write!(f, "Invalid app_id"),
            WeChatError::InvalidBase64(ref err) => err.fmt(f),
            WeChatError::ClientError { errcode, ref errmsg } => write!(f, "Client error code: {}, message: {}", errcode, errmsg),
            WeChatError::IOError(ref err) => err.fmt(f),
            WeChatError::InvalidValue => write!(f, "Invalid Value of Null"),
            WeChatError::Custom { code, ref msg } => write!(f, "Client error code: {}, message: {}", code, msg),

        }
    }
}

/// 错误信息扩展
impl error::Error for WeChatError {

    /// 错误信息输出
    fn description(&self) -> &str {
        match *self {
            WeChatError::InvalidSignature => "Invalid signature",
            WeChatError::InvalidAppId => "Invalid app_id",
            WeChatError::InvalidBase64(ref _err) => "Invalid Base64",
            WeChatError::ClientError { ref errmsg, .. } => errmsg,
            WeChatError::IOError(ref _err) => "Invalid IOError",
            WeChatError::InvalidValue => "Invalid Value Of Empty",
            WeChatError::Custom { ref msg, .. } => msg,
        }
    }
}

impl From<DecodeError> for WeChatError {
    fn from(err: DecodeError) -> WeChatError {
        WeChatError::InvalidBase64(err)
    }
}

impl From<io::Error> for WeChatError {
    fn from(err: io::Error) -> WeChatError {
        WeChatError::IOError(err)
    }
}



impl WeChatError {
    /// Creates generic error
    pub fn msg(value: impl ToString) -> Self {
        WeChatError::InvalidAppId
    }

    /// 自定义错误
    pub fn custom(code: i32, msg: impl ToString) -> Self {
         WeChatError::Custom {
                code,
                msg: msg.to_string(),
            }
    }

}