//! copyright © ecdata.cn 2021 - present
//! 微信处理错误信息处理

use std::fmt;
use std::error;
use std::io;
use base64::DecodeError;

/// 微信处理错误
#[derive(Debug)]
pub enum WechatError {
    
    InvalidSignature,
    InvalidAppId,
    InvalidBase64(DecodeError),
    ClientError { errcode: i32, errmsg: String },
    IOError(io::Error),
    InvalidValue,
    Custom { code: i32, msg: String },
}

/// 实现调试输出
impl fmt::Display for WechatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WechatError::InvalidSignature => write!(f, "Invalid signature"),
            WechatError::InvalidAppId => write!(f, "Invalid app_id"),
            WechatError::InvalidBase64(ref err) => err.fmt(f),
            WechatError::ClientError { errcode, ref errmsg } => write!(f, "Client error code: {}, message: {}", errcode, errmsg),
            WechatError::IOError(ref err) => err.fmt(f),
            WechatError::InvalidValue => write!(f, "Invalid Value of Null"),
            WechatError::Custom { code, ref msg } => write!(f, "Client error code: {}, message: {}", code, msg),

        }
    }
}

/// 错误信息扩展
impl error::Error for WechatError {

    /// 错误信息输出
    fn description(&self) -> &str {
        match *self {
            WechatError::InvalidSignature => "Invalid signature",
            WechatError::InvalidAppId => "Invalid app_id",
            WechatError::InvalidBase64(ref _err) => "Invalid Base64",
            WechatError::ClientError { ref errmsg, .. } => errmsg,
            WechatError::IOError(ref _err) => "Invalid IOError",
            WechatError::InvalidValue => "Invalid Value Of Empty",
            WechatError::Custom { ref msg, .. } => msg,
        }
    }
}

impl From<DecodeError> for WechatError {
    fn from(err: DecodeError) -> WechatError {
        WechatError::InvalidBase64(err)
    }
}

impl From<io::Error> for WechatError {
    fn from(err: io::Error) -> WechatError {
        WechatError::IOError(err)
    }
}



impl WechatError {
    /// Creates generic error
    pub fn msg(_value: impl ToString) -> Self {
        WechatError::InvalidAppId
    }

    /// 自定义错误
    pub fn custom(code: i32, msg: impl ToString) -> Self {
         WechatError::Custom {
                code,
                msg: msg.to_string(),
            }
    }

}