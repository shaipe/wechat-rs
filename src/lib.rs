//! copyright © shaipe 2020 - persent
//! 

pub mod config;

pub mod client;

pub mod errors;

pub mod types;

pub mod wechat_crypto;

pub mod xmlutil;

pub mod tripartite;
pub use tripartite::{WechatTicket,TripartiteConfig};

// #[macro_use]
// extern crate lazy_static;