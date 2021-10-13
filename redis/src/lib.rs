//! copyright © ecdata.cn 2021 - present
//! Redis操作相关封装
//! created by shaipe 20211013

#[macro_use]
extern crate wechat_sdk;
use wechat_sdk::WechatError;

// 导入宏
#[macro_use]
extern crate lazy_static;
// pub use xmlutil::
mod session;
pub use session::{RedisStorage, SessionStore};

mod config;
pub use config::{get_redis_conf, set_redis_conf, RedisConfig};


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
