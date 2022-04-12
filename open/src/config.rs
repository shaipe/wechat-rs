//! copyright
//! 微信三方信息配置

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use std::fs::File;
use std::io::prelude::*;

///tripartite 配置
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Config {
    pub id: u32,
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
    // aeskey
    pub encoding_aes_key: String,
    // 隐私
    pub privacy_json_path: String,
    // 扩展json
    pub ext_json_path: String,
    // 业务域名
    pub web_view_domain: String,
    // 请求域名
    pub request_domain: String,
    // api域名
    pub api_domain: String,
    // 灰度发布域名
    pub api_gray_domain: String,
}

impl Config {
    /// 创建一个新的配置对象
    pub fn new(yaml_doc: yaml_rust::yaml::Yaml) -> Self {
        Config {
            id: 0,
            name: yaml_doc["name"].as_str().unwrap_or("").to_owned(),
            domain: yaml_doc["domain"].as_str().unwrap_or("").to_owned(),
            app_id: yaml_doc["app_id"].as_str().unwrap_or("").to_owned(),
            secret: yaml_doc["secret"].as_str().unwrap_or("").to_owned(),
            token: yaml_doc["token"].as_str().unwrap_or("").to_owned(),
            privacy_json_path: yaml_doc["privacy_json_path"]
                .as_str()
                .unwrap_or("")
                .to_owned(),
            ext_json_path: yaml_doc["ext_json_path"].as_str().unwrap_or("").to_owned(),
            encoding_aes_key: yaml_doc["encoding_aes_key"]
                .as_str()
                .unwrap_or("")
                .to_owned(),
            web_view_domain: yaml_doc["web_view_domain"]
                .as_str()
                .unwrap_or("")
                .to_owned(),
            request_domain: yaml_doc["request_domain"].as_str().unwrap_or("").to_owned(),
            api_domain: yaml_doc["api_domain"].as_str().unwrap_or("").to_owned(),
            api_gray_domain: yaml_doc["api_gray_domain"]
                .as_str()
                .unwrap_or("")
                .to_owned(),
        }
    }
}

// 默认加载静态全局
lazy_static! {
    pub static ref TRIPARTITE_CACHES: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::default()));
}

/// 将配置写入缓存
pub fn set_tripartite_config(cnf: Config) {
    let counter = Arc::clone(&TRIPARTITE_CACHES);
    let mut cache = counter.lock().unwrap();
    *cache = cnf;
}

/// 从缓存中取出第三方配置信息
pub fn get_tripartite_config() -> Config {
    let mut cache = match Arc::clone(&TRIPARTITE_CACHES).lock() {
        Ok(s) => s.clone(),
        Err(_) => Config::default(),
    };
    if cache.app_id.len() == 0 {
        let cnf = read_tripartite_config();

        set_tripartite_config(cnf.clone());
        cache = cnf;
    }
    cache
}

/// 获取可修改的三方配置
pub fn get_tripartite_config_mut<F>(mut func: F) -> Config
where
    F: FnMut() -> Config,
{
    let mut cache = match Arc::clone(&TRIPARTITE_CACHES).lock() {
        Ok(s) => s.clone(),
        Err(_) => Config::default(),
    };
    if cache.app_id.len() == 0 {
        let cnf = func();
        set_tripartite_config(cnf.clone());
        cache = cnf;
    }
    cache
}

/// 获取配置信息
pub fn read_tripartite_config() -> Config {
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
    let doc = yaml_rust::yaml::YamlLoader::load_from_str(&str_val).unwrap();
    let yaml_doc = doc[0].clone();
    Config::new(yaml_doc)
}
