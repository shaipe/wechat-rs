//! copyright © ecdata.cn 2021 - present
//! 公众号对接开发应用
//! created by shaipe 20210302



#[macro_use]
extern crate wechat_sdk;

// use wechat_sdk::WechatError;

#[macro_use]
extern crate serde_json;


pub mod message;

pub mod authorize;
pub use authorize::WechatAuthorize;