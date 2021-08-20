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

// 请求默认AGENT
pub const DEFAULT_USER_AGENT: &'static str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3534.4 Safari/537.36";


#[cfg(feature="req_async")]
mod reqw_client;
#[cfg(feature="req_async")]
pub use reqw_client::Client;
#[cfg(feature="actix")]
mod actix_client;
#[cfg(feature="actix")]
pub use actix_client::Client;

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


///
#[inline]
pub fn json_decode(data: &str) -> WechatResult<serde_json::Value> {
    let obj: serde_json::Value = match serde_json::from_str(data) {
        Ok(decoded) => decoded,
        Err(ref e) => {
            return Err(error! {
                code: -3,
                msg: format!("Json decode error: {}", e),
            });
        }
    };
    let code = match obj["code"].as_i64() {
        Some(v) => v,
        None => 0,
    };
    if code != 0 {
        let msg: String = obj["msg"].to_string();
        return Err(error! {
            code: code as i32,
            msg: msg,
        });
    }
    println!("obj====={:?}", obj);
    Ok(obj)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
