//! copyright
//! 微信三方信息配置

use serde::{Deserialize, Serialize};
use std::sync::Mutex;

///tripartite 配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TripartiteConfig {
    pub name: String,
    pub domain: String,
    pub app_id: String,
    pub secret: String,
    pub token: String,
    pub encoding_aes_key: String,
    pub wap_domain: String,
    pub webview_domain: String,
    pub request_domain: String,
    pub extjson: String,
}

impl TripartiteConfig {
    pub fn default() -> Self {
        TripartiteConfig {
            name: String::from(""),
            domain: String::from(""),
            app_id: String::from(""),
            secret: String::from(""),
            token: String::from(""),
            encoding_aes_key: String::from(""),
            wap_domain: String::from(""),
            webview_domain: String::from(""),
            request_domain: String::from(""),
            extjson: String::from(""),
        }
    }
}

// // 默认加载静态全局
lazy_static! {
    pub static ref TRIPARTITE_CACHES: Mutex<TripartiteConfig> =
        Mutex::new(TripartiteConfig::default());
}

/// 将配置写入缓存
pub fn set_tripartite_config(cnf: TripartiteConfig) {
    let mut cache = TRIPARTITE_CACHES.lock().unwrap();
    *cache = cnf;
}

/// 从缓存中取出第三方配置信息
pub fn get_tripartite_config() -> TripartiteConfig {
    let cache = TRIPARTITE_CACHES.lock().unwrap();
    cache.clone()
}
