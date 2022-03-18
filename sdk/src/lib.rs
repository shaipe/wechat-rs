//! copyright © ecdata.cn 2021 - present
//! 微信系列对接处理公用工具类
//! created by shaipe 20210228

/// 字义微信结果类型
pub type WechatResult<T> = Result<T, WechatError>;

#[macro_use]
pub mod macros;

#[macro_use]
extern crate lazy_static;

// 错误信息处理定义
mod errors;
pub use errors::{WechatError,ErrorKind};

// 加解密处理
mod wxcrypto;
pub use wxcrypto::{WeChatCrypto};

// 公共AccessToken管理
mod token;
pub use token::AccessToken;

// 导出常量配置
pub mod constant;

#[cfg(feature="req_async")]
mod reqw_client;
#[cfg(feature="req_async")]
pub use reqw_client::Client;
#[cfg(feature="actix")]
mod actix_client;
#[cfg(feature="actix")]
pub use actix_client::Client;

pub mod config;
pub use config::*;

pub mod xmlutil;
pub mod aes_crypt;
pub use aes_crypt::AesCrypt;

/// 写入文件到日志
// #[allow(dead_code)]
pub fn write_to_file(conf_path: &str, content: String) -> WechatResult<bool> {
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    // 以读,写,创建,追加的方式打开文件
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        // .append(true)
        .open(conf_path);

    // 向文件中写入内容
    match file {
        Ok(mut stream) => match stream.write_all(content.as_bytes()) {
            Ok(_) => Ok(true),
            Err(err) => Err(error!("{:?}", err)),
        },
        Err(err) => Err(error!("{:?}", err)),
    }
}

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
