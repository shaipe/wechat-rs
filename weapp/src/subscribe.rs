//! copyright © ecdata.cn 2021 - present
//! 订阅消息
//! created by shaipe 20210228
//! https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/subscribe-message/subscribeMessage.addTemplate.html
//! addTemplate
//! deleteTemplate
//! getCategory
//! getPubTemplateKeyWordsById
//! getPubTemplateTitleList
//! getTemplateList
//! send
use wechat_sdk::{Client, WechatResult};

pub struct Subscribe {
    auth_access_token: String,
}
impl Subscribe {
    pub fn new(_auth_access_token: &str) -> Self {
        Subscribe {
            auth_access_token: _auth_access_token.to_string(),
        }
    }
    /// 发送订阅消息
    pub async fn send_sub_message(&self,open_id:&str,template_id:&str,template_data:&serde_json::Value,detail_url:&str) -> WechatResult<bool> {
        let uri = format!(
            "{}{}",
            crate::API_DOMAIN,
            format!(
                "/cgi-bin/message/subscribe/send?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let req_data=json!({
            "touser":open_id,
            "template_id":template_id,
            "page":detail_url,
            "data":template_data,
            "miniprogram_state":"developer"
        });
        log!("data=={:?}",req_data);
        let api = Client::new();
        let res = api.post(&uri, &req_data).await?;
        let _ = wechat_sdk::json_decode(&res)?;
        Ok(true)
    }
}