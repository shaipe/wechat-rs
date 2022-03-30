//! copyright © ecdata.cn 2021 - present
//! 微信接口调用时的文件配置信息
use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConfig {
    pub server: String,
    pub password: String,
    pub dbid: i64,
    pub port: i64,
}
impl RedisConfig {
    pub fn default() -> Self {
        RedisConfig {
            server: String::from(""),
            password: String::from(""),
            dbid: 0,
            port: 6379,
        }
    }
    pub fn new(yaml_doc:yaml_rust::yaml::Yaml)->Self{
        let server=format!("{:?}",yaml_doc["server"].as_str().unwrap_or(""));
        let password=format!("{:?}",yaml_doc["password"].as_str().unwrap_or(""));
        let dbid=yaml_doc["dbid"].as_i64().unwrap_or(0);
        let port=yaml_doc["port"].as_i64().unwrap_or(0);
        RedisConfig{
            server:server,
            password: password,
            dbid:dbid,
            port: port
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

    let mut cache = match Arc::clone(&REDIS_TICKET_CACHES).lock(){
        Ok(s)=>{s.clone()},
        Err(_)=>{
            RedisConfig::default()
        }
    };
    if cache.server.len()==0{
        let cnf= read_redis_config();
     
        set_redis_conf(cnf.clone());
        cache=cnf;
    }
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
    let doc=yaml_rust::yaml::YamlLoader::load_from_str(&str_val).unwrap();
    let yaml_doc=doc[0].clone();
    RedisConfig::new(yaml_doc)
}