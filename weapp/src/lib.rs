//! copyright © ecdata.cn 2021 - present
//! 微信小程序功能对接
//! created by shaipe 20210228

// mod weapp;
// pub use weapp::WeApp;

#[macro_use]
extern crate wechat_sdk;
use wechat_sdk::WechatError;

// 引入json宏
#[macro_use]
extern crate serde_json;

mod auth;
pub use auth::Auth;

mod qrcode;
pub use qrcode::QRCode;

mod config;
pub use config::WeappConfig;



// 接口域名
pub(crate) const API_DOMAIN: &'static str = "https://api.weixin.qq.com";