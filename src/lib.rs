//! copyright © shaipe 2020 - persent
//!

pub mod errors;

pub use errors::WeChatError;

/// 字义微信结果类型
pub type WeChatResult<T> = Result<T, errors::WeChatError>;

pub mod config;

pub mod client;

pub mod message;

pub mod wechat_crypto;
pub use wechat_crypto::WeChatCrypto;

pub mod xmlutil;

pub mod tripartite;
pub use tripartite::{
    get_tripartite_config, set_tripartite_config, Ticket, TripartiteConfig, WechatComponent,
};
pub mod official;
pub use official::WechatAuthorize;

#[macro_use]
extern crate lazy_static;
use rustc_serialize::json::Json;

#[inline]
pub fn json_decode(data: &str) -> WeChatResult<Json> {
    let obj = match Json::from_str(data) {
        Ok(decoded) => decoded,
        Err(ref e) => {
            return Err(WeChatError::ClientError {
                errcode: -3,
                errmsg: format!("Json decode error: {}", e),
            });
        }
    };
    match obj.find("errcode") {
        Some(code) => {
            let errcode = code.as_i64().unwrap();
            if errcode != 0 {
                let errmsg = match obj.find("errmsg") {
                    Some(msg) => msg.as_string().unwrap(),
                    None => "",
                };
                return Err(WeChatError::ClientError {
                    errcode: errcode as i32,
                    errmsg: errmsg.to_owned(),
                });
            }
        }
        None => {}
    };
    Ok(obj)
}
