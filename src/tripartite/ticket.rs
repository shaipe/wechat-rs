use super::config::TripartiteConfig;
use crate::tripartite::component::WechatComponent;
use crate::types::WeChatResult;
use crate::wechat_crypto::WeChatCrypto;
use crate::xmlutil;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::errors::WeChatError;
use std::sync::Mutex;

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
    
    /// 解析ticket
    pub fn parse_ticket(
        conf: TripartiteConfig,
        xml: &str,
        query_params: HashMap<String, String>,
    ) -> WeChatResult<String> {
        //随机数
        let nonce = get_hash_value(&query_params, "nonce");
        //时间缀
        let timestamp = match get_hash_value(&query_params, "timestamp").parse::<i64>() {
            Ok(v) => v,
            Err(_e) => 0,
        };
        //签名信息
        let signature = get_hash_value(&query_params, "msg_signature");
        // println!("{:?}", conf);
        
        let c = WeChatCrypto::new(&conf.token, &conf.encoding_aes_key, &conf.app_id);
        // let decrpty = c.decrypt_message(xml, &signature, timestamp, &nonce);
        match c.decrypt_message(xml, &signature, timestamp, &nonce) {
            Ok(v) => { 
                let package = xmlutil::parse(v);
                let doc = package.as_document();
                let ticketstr = xmlutil::evaluate(&doc, "//xml/ComponentVerifyTicket/text()").string();
                Ok(ticketstr)
            }
            Err(_) => {
                Err(WeChatError::InvalidSignature)
            }
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
            let c = WechatComponent::new(&conf.app_id, &conf.secret, &self.access_ticket);
            let result = c.fetch_access_token().await;
            match result {
                Ok(token) => {
                    println!("token={:?}", token);
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

fn get_hash_value(query_params: &HashMap<String, String>, key: &str) -> String {
    match query_params.get(key) {
        Some(val) => val.clone(),
        None => "".to_owned(),
    }
}


// 默认加载静态全局
lazy_static! {
    pub static ref TRIPARTITE_TICKET_CACHES: Mutex<Ticket> =
        Mutex::new(Ticket::default());
}

/// 设置ticket
pub fn set_ticket(cnf: Ticket) {
    let mut cache = TRIPARTITE_TICKET_CACHES.lock().unwrap();
    *cache = cnf;
}

/// 获取ticket
pub fn get_ticket() -> Ticket {
    let cache = TRIPARTITE_TICKET_CACHES.lock().unwrap();
    cache.clone()
}