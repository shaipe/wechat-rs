//! copyright 
//! 微信第三方平台的ticket获取存储

use super::{Component, TripartiteConfig};
use crate::errors::WeChatError;

use crate::wechat_crypto::WeChatCrypto;
use crate::xmlutil;
use crate::WeChatResult;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Mutex,Arc};
use std::time::{SystemTime, UNIX_EPOCH};

/// Ticket对象
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ticket {
    pub access_ticket: String,
    pub ticket_time: i64,
    pub access_token: String,
    pub at_expired_time: i64,
}

impl Default for Ticket {
    fn default() -> Self {
        Ticket {
            access_ticket: String::from(""),
            ticket_time: 0,
            access_token: String::from(""),
            at_expired_time: 0,
        }
    }
}

impl Ticket {
    /// 加载配置
    pub fn new(config_path: &str) -> Self {
        let file_path = if config_path.is_empty() {
            "ticket.conf"
        } else {
            config_path
        };

        // 如果没有配置ticket文件,返回默认值
        if !std::path::Path::new(file_path).exists() {
            return Ticket::default();
        }

        // 打开文件
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => {
                println!("no such file {} exception: {}", file_path, e);
                return Ticket::default();
            }
        };

        // 读取文件到字符串变量
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => {
                println!("Error Reading file:{}", e);
                return Ticket::default();
            }
        };

        let cnf = serde_json::from_str(&str_val);
        // println!("{:?}", cnf);
        // 第三方配置处理
        match cnf {
            Ok(val) => {
                let t:Ticket=val;
                set_ticket(t.clone());
                t
            }
            Err(e) => {
                println!("Ticket文件配置错误! {:?}", e);
                Ticket::default()
            }
        }
    }

    /// 解析ticket
    pub fn parse_ticket(
        conf: TripartiteConfig,
        xml: &str,
        query_params: HashMap<String, String>,
    ) -> WeChatResult<String> {
        let c = WeChatCrypto::new(&conf.token, &conf.encoding_aes_key, &conf.app_id);
        // let decrpty = c.decrypt_message(xml, &signature, timestamp, &nonce);
        match c.decrypt_message(xml, &query_params) {
            Ok(v) => {
                let package = xmlutil::parse(v);
                let doc = package.as_document();
                let ticketstr =
                    xmlutil::evaluate(&doc, "//xml/ComponentVerifyTicket/text()").string();
                Ok(ticketstr)
            }
            Err(_) => Err(WeChatError::InvalidSignature),
        }
    }

    /// 保存ticket到文件
    pub fn save(&self, config_path: &str) {
        let file_path = if config_path.is_empty() {
            "ticket.conf"
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
        println!("path={:?}", str_val);
        match file.write_all(str_val.as_bytes()) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file:{}", e),
        };
    }

    /// 获取access_token
    pub async fn get_token(&mut self, conf: TripartiteConfig) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let expires_at: i64 = self.at_expired_time;
        //比较过期时间
        if expires_at <= timestamp {
            let c = Component::new(conf);
            let result = c.fetch_access_token().await;
            // println!("result={:?},access_ticket={:?}", result, self.access_ticket);
            match result {
                Ok(token) => {
                    self.access_token = token.0.clone();
                    self.at_expired_time = token.1;
                    set_ticket(self.clone());
                    self.save("");
                    token.0
                }
                Err(_) => "".to_owned(),
            }
        } else {
            self.access_token.clone()
        }
    }
}

// 默认加载静态全局
lazy_static! {
    pub static ref TRIPARTITE_TICKET_CACHES: Arc<Mutex<Ticket>>= Arc::new(Mutex::new(Ticket::default()));
}

/// 设置ticket
pub fn set_ticket(cnf: Ticket) {
    let counter=Arc::clone(&TRIPARTITE_TICKET_CACHES);
    let mut cache = counter.lock().unwrap();
    *cache = cnf;
}

/// 获取ticket
pub fn get_ticket() -> Ticket {
    let counter=Arc::clone(&TRIPARTITE_TICKET_CACHES);
    let cache = counter.lock().unwrap();
    cache.clone()
}
