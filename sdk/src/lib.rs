//! copyright © ecdata.cn 2021 - present
//! 微信系列对接处理公用工具类
//! created by shaipe 20210228

/// 字义微信结果类型
pub type WechatResult<T> = Result<T, WechatError>;

#[macro_use]
pub mod macros;

mod errors;
pub use errors::WechatError;

mod wxcrypto;
pub use wxcrypto::{aes128_cbc_decrypt, aes256_cbc_decrypt, WeChatCrypto};

mod client;
pub use client::Client;

pub mod xmlutil;
// pub use xmlutil::
mod session;
pub use session::{RedisStorage, SessionStore};

#[macro_use]
extern crate lazy_static;

mod config;
pub use config::{get_redis_conf, set_redis_conf, RedisConfig};
/// 获取当前时间戮
pub fn current_timestamp() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
