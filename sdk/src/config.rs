//! copyright © ecdata.cn 2021 - present
//! 微信接口调用时的文件配置信息

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConfig {
    pub server: String,
    pub password: String,
    pub dbid: i32,
    pub port: i32,
}
impl RedisConfig {
    pub fn default() -> Self {
        RedisConfig {
            server: String::from("redis://127.0.0.1"),
            password: String::from(""),
            dbid: 0,
            port: 6379,
        }
    }
}

use std::sync::{Arc, Mutex};
// 默认加载静态全局
lazy_static! {
    pub static ref REDIS_TICKET_CACHES: Arc<Mutex<RedisConfig>> =
        Arc::new(Mutex::new(RedisConfig::default()));
}

/// 设置ticket
pub fn set_redis_conf(cnf: RedisConfig) {
    let counter = Arc::clone(&REDIS_TICKET_CACHES);
    let mut cache = counter.lock().unwrap();
    *cache = cnf;
}

/// 获取ticket
pub fn get_redis_conf() -> RedisConfig {
    let counter = Arc::clone(&REDIS_TICKET_CACHES);
    let cache = counter.lock().unwrap();
    cache.clone()
}
