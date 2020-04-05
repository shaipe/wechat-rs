use std::collections::HashMap;
use crate::types::WeChatResult;
use crate::client::Client;
use crate::errors::WeChatError;
use crate::config::Config;
const WECHAT_URI: &'static str = "https://api.weixin.qq.com";
pub struct WechatComponent{
    app_id:String,
    secret:String,
    ticket:String
}
impl WechatComponent{
    pub fn new(_app_id:&str,_secret:&str,_ticket:&str) -> WechatComponent{
        WechatComponent{
            app_id:_app_id.to_string(),
            secret:_secret.to_string(),
            ticket:_ticket.to_string()
        }
    }
    pub async fn get_component_token(&self) -> WeChatResult<String>{
        let uri=format!("{}{}",WECHAT_URI,"/cgi-bin/component/api_component_token");
        let mut hash=HashMap::new();
        hash.insert("component_appid".to_string(),self.app_id.clone());
        hash.insert("component_appsecret".to_string(),self.secret.clone());
        hash.insert("component_verify_ticket".to_string(),self.ticket.clone());
        let c=Config::default();
        let api=Client::new(c);
        api.post(&uri,&hash).await
    }
}