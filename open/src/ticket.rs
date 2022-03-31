//! copyright
//! 微信第三方平台的ticket获取存储

use super::TripartiteConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wechat_redis::RedisConfig;
use wechat_sdk::{xmlutil, WeChatCrypto, WechatResult};
/// Ticket对象
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ticket {
    pub tripart_conf: TripartiteConfig,
    pub redis_con: String,
}

// impl Default for Ticket {
//     fn default() -> Self {
//         Ticket {
//             redis_con: String::from("")
//         }
//     }
// }

impl Ticket {
    pub fn new(tripart_conf: TripartiteConfig, redis_conf: RedisConfig) -> Self {
        let redis_con = format!(
            "redis://:{}@{}:{}/{}",
            &redis_conf.password, &redis_conf.server, &redis_conf.port, redis_conf.dbid
        );
        println!("redis_con={}", redis_con);
        Ticket {
            redis_con: redis_con,
            tripart_conf: tripart_conf,
        }
    }
    /// 解析ticket
    pub fn parse_ticket(
        &self,
        xml: &str,
        query_params: HashMap<String, String>,
    ) -> WechatResult<String> {
        let c = WeChatCrypto::new(
            &self.tripart_conf.token,
            &self.tripart_conf.encoding_aes_key,
            &self.tripart_conf.app_id,
        );
        match c.decrypt_message(xml, &query_params) {
            Ok(v) => {
                let package = xmlutil::parse(v);
                let doc = package.as_document();
                let ticketstr =
                    xmlutil::evaluate(&doc, "//xml/ComponentVerifyTicket/text()").string();

                set_ticket_cache(
                    &self.redis_con,
                    &self.tripart_conf.app_id,
                    ticketstr.clone(),
                );
                Ok(ticketstr)
            }
            Err(_) => Err(error! {code: 3000, msg: "Invalid"}),
        }
    }
    //获取ticket
    pub fn get_ticket(&self) -> String {
        get_ticket_cache(&self.redis_con, &self.tripart_conf.app_id)
    }
}
pub const APP_TICKET_CACHES: &str = "APP_TICKET_CACHES";
use wechat_redis::{RedisStorage, SessionStore};
/// 批量设置
pub fn set_ticket_cache(redis_con: &str, key: &str, v: String) {
    let cache_key = format!("{0}_{1}", APP_TICKET_CACHES, key);
    match RedisStorage::from_url(redis_con) {
        Ok(session) => {
            session.set(cache_key, v, Some(10 * 60 * 60));
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
/// 获取
pub fn get_ticket_cache(redis_con: &str, key: &str) -> String {
    let cache_key = format!("{}_{}", APP_TICKET_CACHES, key);
    let d = "".to_owned();
    match RedisStorage::from_url(redis_con) {
        Ok(session) => {
            if let Some(v) = session.get(cache_key, "get".to_owned(), None) {
                v
            } else {
                d
            }
        }
        Err(_) => d,
    }
}
