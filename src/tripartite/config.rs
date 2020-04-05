use serde_derive::Deserialize;
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
    pub at_expired_time: String,
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
            at_expired_time: String::from(""),
            wap_domain: String::from(""),
            webview_domain: String::from(""),
            request_domain: String::from(""),
            extjson: String::from(""),
        }
    }
    pub async fn get_token(&self) ->String {
        let c=WechatComponent::new(&self.app_id,&self.secret,&self.access_ticket);
        let result=c.get_component_token().await;
        match result{
            Ok(token) => token,
            Err(err) =>"".to_owned()
        }
    }
}
