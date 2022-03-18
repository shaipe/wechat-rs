//! copyright
//! 微信三方信息配置

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use std::fs::File;
use std::io::prelude::*;

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
        }
    }
}
// // 默认加载静态全局
lazy_static! {
        pub static ref TRIPARTITE_CACHES: Arc<Mutex<TripartiteConfig>> =
        Arc::new(Mutex::new(TripartiteConfig::default()));
}

/// 将配置写入缓存
pub fn set_tripartite_config(cnf: TripartiteConfig) {
    let counter = Arc::clone(&TRIPARTITE_CACHES);
    let mut cache = counter.lock().unwrap();
    *cache = cnf;

}

/// 从缓存中取出第三方配置信息
pub fn get_tripartite_config() -> TripartiteConfig {
    let cache = match Arc::clone(&TRIPARTITE_CACHES).lock(){
        Ok(s)=>{s.clone()},
        Err(e)=>{
            let cnf= read_tripartite_config();
            set_tripartite_config(cnf.clone());
            cnf
        }
    };
     cache
    
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
