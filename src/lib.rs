//! copyright © shaipe 2020 - persent
//!


#[macro_use]
extern crate lazy_static;

/// 字义微信结果类型
pub type WeChatResult<T> = Result<T, WeChatError>;

mod errors;
pub use errors::WeChatError;

mod wechat_crypto;
pub use wechat_crypto::WeChatCrypto;


mod client;
pub(crate) use client::Client;

pub mod xmlutil;
pub mod config;
pub mod message;

/// 小程序功能对接模块
pub mod weapp;

/// 第三方开发平台模块
pub mod tripartite;

/// 公众号对接模块
pub mod official;

/// 获取当前时间戮
pub fn current_timestamp() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}
