//! copyright © shaipe 2020 - persent
//!

// #[macro_use]
// extern crate wechat_sdk;
// use wechat_sdk::WechatError;




pub mod open {
    #[cfg(feature = "open")]
    pub use wechat_open::*;
}
/// 微信公众号
pub mod mp {
    #[cfg(feature = "mp")]
    pub use wechat_mp::*;
}

/// 微信小程序
pub mod weapp {
    #[cfg(feature = "weapp")]
    pub use wechat_weapp::*;
}

/// 微信支付
pub mod pay {
    #[cfg(feature = "pay")]
    pub use wechat_weapp::*;
}

/// 微信小店
pub mod store {
    #[cfg(feature = "store")]
    pub use wechat_store::*;
}

/// 企业微信
pub mod work {
    #[cfg(feature = "work")]
    pub use wechat_work::*;
}

mod config;
pub use config::Config;

pub use wechat_sdk::*;