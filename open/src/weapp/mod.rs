//! copyright © ecdata.cn 2022 - present
//! 基于开放平台的微信小程序处理

// 小程序登录
mod auth;
pub use auth::*;

// 小程序类目管理
mod category;
pub use category::*;

// 小程序代码管理
mod code;
pub use code::*;

// 小程序隐私管理
mod privacy;
pub use privacy::Privacy;

mod tester;
pub use tester::*;

// 小程序基础信息
mod basic;
pub use basic::Basic;

// 小程序域名管理
mod domain;
pub use domain::Domain;

mod template;
pub use template::Template;

mod official;
pub use official::Official;

// 直播
mod live;
pub use live::Live;

// 物流
mod express;
pub use express::Express;

// 购物订单
mod order;
pub use order::Order;

