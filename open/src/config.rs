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
    pub fn new(yaml_doc:yaml_rust::yaml::Yaml)->Self{
        let name=format!("{:?}",yaml_doc["name"].as_str().unwrap_or(""));
        let domain=format!("{:?}",yaml_doc["domain"].as_str().unwrap_or(""));
        let app_id=format!("{:?}",yaml_doc["app_id"].as_str().unwrap_or(""));
        let secret=format!("{:?}",yaml_doc["secret"].as_str().unwrap_or(""));
        let token=format!("{:?}",yaml_doc["token"].as_str().unwrap_or(""));
        let encoding_aes_key=format!("{:?}",yaml_doc["encoding_aes_key"].as_str().unwrap_or(""));
        TripartiteConfig{
            name:name,
            domain: domain,
            app_id:app_id,
            secret: secret,
            token: token,
            encoding_aes_key: encoding_aes_key
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
    let mut cache = match Arc::clone(&TRIPARTITE_CACHES).lock(){
        Ok(s)=>{
            s.clone()
        },
        Err(_)=>{
            TripartiteConfig::default()
        }
    };
    if cache.app_id.len()==0{
        let cnf= read_tripartite_config();
     
        set_tripartite_config(cnf.clone());
        cache=cnf;
    }
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
    let doc=yaml_rust::yaml::YamlLoader::load_from_str(&str_val).unwrap();
    let yaml_doc=doc[0].clone();
    TripartiteConfig::new(yaml_doc)
}
