use serde_derive::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;

use crate::tripartite::component::WechatComponent;

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
