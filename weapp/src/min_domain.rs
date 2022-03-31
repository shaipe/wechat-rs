//! copyright © ecdata.cn 2021 - present
//! 小程序域名设置

use wechat_sdk::{Client, WechatResult};

use std::collections::HashMap;

const API_DOMAIN: &'static str ="https://api.weixin.qq.com";

pub struct MinDomain {
    authorizer_access_token: String,
}
impl MinDomain {

    pub fn new( _authorizer_access_token: &str) -> Self {
        MinDomain {
            authorizer_access_token: _authorizer_access_token.to_string(),
        }
    }
    /// 设置服务器域名
    pub async fn set_server_domain(&self,req_domain:Vec<String>,ws_domain:Vec<String>,upload_domain:Vec<String>,down_domain:Vec<String>)->WechatResult<serde_json::Value>{
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/modify_domain?access_token={}",
                self.authorizer_access_token.clone()
            )
        );

        let mut hash = HashMap::new();
        hash.insert("action".to_string(),"set".to_owned());
        hash.insert("requestdomain".to_string(),format!("{}",req_domain.join(";")));
        hash.insert("wsrequestdomain".to_string(),format!("{}",ws_domain.join(";")));
        hash.insert("uploaddomain".to_string(),format!("{}",upload_domain.join(";")));
        hash.insert("downloaddomain".to_string(),format!("{}",down_domain.join(";")));
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;
        wechat_sdk::json_decode(&res)
    }
    /// 设置业务域名
    pub async fn set_webview_domain(&self,req_domain:Vec<String>)->WechatResult<serde_json::Value>{
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/setwebviewdomain?access_token={}",
                self.authorizer_access_token.clone()
            )
        );

        let mut hash = HashMap::new();
        hash.insert("action".to_string(),"set".to_owned());
        hash.insert("webviewdomain".to_string(),format!("{}",req_domain.join(";")));
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;
        wechat_sdk::json_decode(&res)
    }
}
