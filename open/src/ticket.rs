//! copyright
//! 微信第三方平台的ticket获取存储

use super::TripartiteConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wechat_sdk::{xmlutil, WeChatCrypto, WechatResult};
/// Ticket对象
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ticket {
    pub tripart_conf: TripartiteConfig
}

// impl Default for Ticket {
//     fn default() -> Self {
//         Ticket {
//             redis_con: String::from("")
//         }
//     }
// }

impl Ticket {
    pub fn new(tripart_conf: TripartiteConfig) -> Self {
        Ticket {
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

                // set_ticket_cache(
                //     &self.redis_con,
                //     &self.tripart_conf.app_id,
                //     ticketstr.clone(),
                // );
                Ok(ticketstr)
            }
            Err(_) => Err(error! {code: 3000, msg: "Invalid"}),
        }
    }
}
