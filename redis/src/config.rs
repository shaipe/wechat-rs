//! copyright © ecdata.cn 2021 - present
//! 微信接口调用时的文件配置信息
use std::fs::File;
use std::io::prelude::*;
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

/// 设置redis config
pub fn set_redis_conf(cnf: RedisConfig) {
    let counter = Arc::clone(&REDIS_TICKET_CACHES);
    let mut cache = counter.lock().unwrap();
    *cache = cnf;
}

/// 获取redis config
pub fn get_redis_conf() -> RedisConfig {

    let cache = match Arc::clone(&REDIS_TICKET_CACHES).lock(){
        Ok(s)=>{s.clone()},
        Err(_)=>{
            let cnf= read_redis_config();
            set_redis_conf(cnf.clone());
            cnf
        }
    };
     cache
}
// 获取配置信息
pub fn read_redis_config() -> RedisConfig {
    // 加载配置文件
    let file_path = "config/redis.yml";

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
    let cnf:RedisConfig =match serde_json::from_str(&str_val){
        Ok(s)=>s,
        Err(e)=>{
            panic!("TripartiteConfig load error: {}", e)
        }
    };
    cnf
}