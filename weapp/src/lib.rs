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

mod min_domain;
pub use min_domain::MinDomain;

mod min_category;
pub use min_category::{MinCategory,MinCategoryItem};

mod min_code;
pub use min_code::{MinCode};

mod min_tester;
pub use min_tester::MinTester;
// 接口域名
pub(crate) const API_DOMAIN: &'static str = "https://api.weixin.qq.com";
