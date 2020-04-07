use serde_derive::{Deserialize,Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use serde_json::json;
use std::fs::File;
use std::io::prelude::*;

use crate::tripartite::component::WechatComponent;
/*
tripartite 配置
*/
#[derive(Debug, Clone, Deserialize,Serialize)]
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
    pub at_expired_time: i64,
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
            at_expired_time: 0,
            wap_domain: String::from(""),
            webview_domain: String::from(""),
            request_domain: String::from(""),
            extjson: String::from(""),
        }
    }
     // 加载配置
     pub fn init(config_path: &str) -> Self {
        let file_path = if config_path.is_empty() {
            "tripartite.conf"
        } else {
            config_path
        };

        // 打开文件
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception: {}", file_path, e),
        };

        // 读取文件到字符串变量
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file:{}", e),
        };
        let cnf = serde_json::from_str(&str_val);
        
        match cnf{
            Ok(val) => {
                let t:TripartiteConfig=val;
                set_tripartite_config(t.clone());
                t
            }
            Err(_) => {
                println!("请配置第三方文件!");
                TripartiteConfig::default()
            }
        }
    }
    // 保存文件
    pub fn save(&self, config_path: &str) {
        let file_path = if config_path.is_empty() {
            "tripartite.conf"
        } else {
            config_path
        };
       
        // 打开文件
        let mut file = match File::create(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception: {}", file_path, e),
        };

        // 读取文件到字符串变量
        let str_val = json!(self).to_string();
        println!("path={:?}",str_val);
        match file.write_all(str_val.as_bytes()) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file:{}", e),
        };
    }
    /**
     * get component access_token
     */
    pub async fn get_token(&mut self) ->String {
        let timestamp =SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        let expires_at: i64 = self.at_expired_time;
        //比较过期时间
        if expires_at <= timestamp {
            let c=WechatComponent::new(&self.app_id,&self.secret,&self.access_ticket);
            let result=c.fetch_access_token().await;
            match result{
                Ok(token) => {
                    println!("token={:?}",token);
                    self.access_token=token.0.clone();
                    self.at_expired_time=token.1;
                    set_tripartite_config(self.clone());
                    self.save("");
                    token.0
                },
                Err(_) =>"".to_owned()
            }
        } else {
            self.access_token.clone()
        }
      
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
    let  cache = TRIPARTITE_CACHES.lock().unwrap();
    cache.clone()
}
