//! copyright © ecdata.cn 2021 - present
//! 微信小程序功能对接
//! created by shaipe 20210228

// mod weapp;
// pub use weapp::WeApp;

#[macro_use]
extern crate wechat_sdk;

use wechat_sdk::WechatError;

mod auth;
pub use auth::Auth;

mod config;
pub use config::WeappConfig;