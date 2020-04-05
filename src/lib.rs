//! copyright © shaipe 2020 - persent
//! 

pub mod config;

pub mod client;

pub mod errors;

pub mod types;

pub mod wechat_crypto;

pub mod xmlutil;

pub mod tripartite;
pub use tripartite::{WechatTicket,TripartiteConfig,WechatComponent};

// #[macro_use]
// extern crate lazy_static;

use reqwest::header;
use std::collections::HashMap;

/// 默认user_agent
const DEFAULT_USER_AGENT: &'static str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3534.4 Safari/537.36";
// Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_3) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.5 Safari/605.1.15


/// 采用post方式请求数据
pub(crate) fn post(url: &str, params: HashMap<String, String>) -> Result<String, std::io::Error> {
    match reqwest::blocking::Client::new()
            .post(url)
            .header(header::USER_AGENT, DEFAULT_USER_AGENT)
            .form(&params)
            .send()
        {
            Ok(res) => {
                if res.status() == 200 {
                    match res.text() {
                        Ok(txt) => Ok(txt),
                        Err(e) => Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("{:?}", e),
                        )),
                    }
                } else {
                    Err(std::io::Error::new(std::io::ErrorKind::Other, "error"))
                }
            }
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )),
        }
}
