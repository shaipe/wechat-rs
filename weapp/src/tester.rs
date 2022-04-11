//! copyright © ecdata.cn 2021 - present
//! 小程序体者管理

use wechat_sdk::{Client, WechatResult};

use std::collections::HashMap;

const API_DOMAIN: &'static str ="https://api.weixin.qq.com";

pub struct Tester {
    authorizer_access_token: String,
}
impl Tester {

    pub fn new( _authorizer_access_token: &str) -> Self {
        Tester {
            authorizer_access_token: _authorizer_access_token.to_string(),
        }
    }
    /// 绑定体验者
    pub async fn bind_tester(&self,wechat_id:&str)->WechatResult<serde_json::Value>{
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/bind_tester?access_token={}",
                self.authorizer_access_token.clone()
            )
        );

        let mut hash = HashMap::new();
        hash.insert("wechatid".to_string(),wechat_id.to_owned());
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;
        wechat_sdk::json_decode(&res)
    }
    /// 解绑体验者
    pub async fn unbind_tester(&self,wechat_id:&str)->WechatResult<serde_json::Value>{
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/unbind_tester?access_token={}",
                self.authorizer_access_token.clone()
            )
        );

        let mut hash = HashMap::new();
        hash.insert("wechatid".to_string(),wechat_id.to_owned());
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;
        wechat_sdk::json_decode(&res)
    }
}
