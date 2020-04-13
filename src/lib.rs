//! copyright © shaipe 2020 - persent
//!

#[macro_use]
extern crate serde_json;

pub mod errors;

pub use errors::WeChatError;

/// 字义微信结果类型
pub type WeChatResult<T> = Result<T, errors::WeChatError>;

pub mod config;

mod client;
pub(crate) use client::Client;

pub mod message;

mod wechat_crypto;
pub use wechat_crypto::WeChatCrypto;

pub mod xmlutil;

pub mod weapp;

pub mod tripartite;
pub use tripartite::{
    get_tripartite_config, set_tripartite_config, Ticket, TripartiteConfig, Component,
};

pub mod official;
pub use official::WechatAuthorize;

#[macro_use]
extern crate lazy_static;

/// 获取当前时间戮
pub fn current_timestamp() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}
