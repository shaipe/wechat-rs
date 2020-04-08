//! copyright © shaipe 2020 - persent
//!

pub mod errors;

/// 字义微信结果类型
pub type WeChatResult<T> = Result<T, errors::WeChatError>;

pub mod config;

pub mod client;

pub mod message;

pub mod types;

pub mod wechat_crypto;

pub mod xmlutil;

pub mod tripartite;
pub use tripartite::{TripartiteConfig,WechatComponent,set_tripartite_config,get_tripartite_config};

#[macro_use]
extern crate lazy_static;


