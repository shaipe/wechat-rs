//! copyright © ecdata.cn 2021 - present
//! 小程序域名设置

use wechat_sdk::{Client, WechatResult};

const API_DOMAIN: &'static str ="https://api.weixin.qq.com";

pub struct Domain {
    authorizer_access_token: String,
}
impl Domain {

    pub fn new( _authorizer_access_token: &str) -> Self {
        Domain {
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

       
        let data=json!({
            "action":"set",
            "requestdomain":req_domain,
            "wsrequestdomain":ws_domain,
            "uploaddomain":upload_domain,
            "downloaddomain":down_domain,
        });

        let api = Client::new();
        let res = api.post(&uri, &data).await?;
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
        let data=json!({
            "action":"set",
            "webviewdomain":req_domain
        });
        let api = Client::new();
        let res = api.post(&uri, &data).await?;
        wechat_sdk::json_decode(&res)
    }
}
