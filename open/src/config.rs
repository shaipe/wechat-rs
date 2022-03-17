//! copyright
//! 微信三方信息配置

use serde::{Deserialize, Serialize};
use std::sync::Mutex;

///tripartite 配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TripartiteConfig {
    // 名称
    pub name: String,
    // 域名
    pub domain: String,
    // 应用id
    pub app_id: String,
    // 密钥
    pub secret: String,
    // 已获取的token
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
    let mut cache = TRIPARTITE_CACHES.lock().unwrap();
     cache.clone()
    
}
use crate::redis::{RedisStorage, SessionStore};
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;
#[derive(Debug, Clone, Deserialize)]
pub struct AppSecret {
    pub name: String,
    pub appid: String,
    pub secret: String,
    pub encrytype: String,
    pub config: Option<BTreeMap<String, String>>,
}
// 获取配置信息
pub fn read_tripartite_config() -> TripartiteConfig {
    // 加载配置文件
    let file_path = "config/tripartite.yml";

    // 打开文件
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            panic!("no such file {} exception: {}", file_path, e)
        }
    };
    // 读取文件到字符串变量
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => {
            panic!("Error Reading file:{}", e);
        }
    };
    let cnf:TripartiteConfig =match serde_json::from_str(&str_val){
        Ok(s)=>s,
        Err(e)=>{
            panic!("TripartiteConfig load error: {}", e)
        }
    };
    cnf
}
const DBID: u16 = 6;
pub const APP_SECRET_CACHES: &str = "APP_SECRET_CACHES";
pub const APP_SECRET_DEFAULT: &str = "096d4009072c927c";
/// 批量设置
pub fn set_ticket_cache(redis_con: &str, cnf: BTreeMap<String, String>) {
    let url = format!("{}/{}", redis_con, DBID);
   
    match RedisStorage::from_url(url) {
        Ok(session) => {
            session.hmset(APP_SECRET_CACHES, cnf.clone());
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
/// 获取
pub fn get_ticket_cache(redis_con: &str) -> BTreeMap<String, String> {
    let d= BTreeMap::new();
    match RedisStorage::from_url(format!("{}/{}", redis_con, DBID)) {
        Ok(session) => {
            if let Some(v) = session.get(APP_SECRET_CACHES, "hgetall", None) {
                v
            } else {
                d
            }
        }
        Err(_) => d,
    }
}