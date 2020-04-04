use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Mutex;

#[derive(Debug, Clone, Deserialize)]
pub struct TripartiteConfig {
    pub name: String,
    pub domain: String,
    pub app_id: String,
    pub secret: String,
    pub token: String,
    pub encoding_aes_key: String,
    pub access_ticket: String,
    pub ticket_time: String,
    pub access_token: String,
    pub at_expired_time: String,
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
            access_ticket: String::from(""),
            ticket_time: String::from(""),
            access_token: String::from(""),
            at_expired_time: String::from(""),
            wap_domain: String::from(""),
            webview_domain: String::from(""),
            request_domain: String::from(""),
            extjson: String::from(""),
        }
    }
}
// 业务配置信息
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub tripartite: Option<TripartiteConfig>,
}

impl Config {
    // 加载配置
    pub fn init(config_path: &str) -> Self {
        let file_path =  if config_path.is_empty() {
            "prod.conf"
        }
        else{
            config_path
        };

        // 打开文件
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception: {}", file_path, e)
        };

        // 读取文件到字符串变量
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file:{}", e)
        };
       
        let cnf: Config = serde_json::from_str(&str_val).unwrap();
        let web_config=match cnf.clone().tripartite{
            Some(val) => {
                set_tripartite_config(val.clone());
                val
            },
            _ => {
                println!("请配置第三方文件!");
                TripartiteConfig::default()
            }
        };
        cnf
    }
}


// // 默认加载静态全局
lazy_static! {
    pub static ref TRIPARTITE_CACHES: Mutex<TripartiteConfig> =
        Mutex::new(TripartiteConfig::default());
}

pub fn set_tripartite_config(cnf: TripartiteConfig) {
    let mut cache = TRIPARTITE_CACHES.lock().unwrap();
    *cache = cnf;
}

pub fn get_tripartite_config() -> TripartiteConfig {
    let mut cache = TRIPARTITE_CACHES.lock().unwrap();
    cache.clone()
}
