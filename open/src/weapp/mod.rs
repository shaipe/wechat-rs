//! copyright © ecdata.cn 2022 - present
//! 基于开放平台的微信小程序处理

mod auth;
pub use auth::*;

mod category;
pub use category::*;

mod code;
pub use code::*;

mod privacy;
pub use privacy::Privacy;

mod tester;
pub use tester::*;

mod basic;
pub use basic::Basic;


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