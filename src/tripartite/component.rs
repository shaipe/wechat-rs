use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use rustc_serialize::json::{Json};

use crate::types::WeChatResult;
use crate::client::Client;
use crate::errors::WeChatError;
use crate::config::Config;
const WECHAT_URI: &'static str = "https://api.weixin.qq.com";
const REFETCH_ACCESS_TOKEN_ERRCODES: [i32; 3] = [40001, 40014, 42001];
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
    /**
     * 获取access_token
     */
    pub async fn fetch_access_token(&self) -> WeChatResult<(String,i64)>{
        let uri=format!("{}{}",WECHAT_URI,"/cgi-bin/component/api_component_token");
        let mut hash=HashMap::new();
        hash.insert("component_appid".to_string(),self.app_id.clone());
        hash.insert("component_appsecret".to_string(),self.secret.clone());
        hash.insert("component_verify_ticket".to_string(),self.ticket.clone());
        let c=Config::default();
        let api=Client::new(c);
        //post
        let result=api.post(&uri,&hash).await?;
      
        let data = match api.json_decode(&result) {
            Ok(_data) => _data,
            Err(err) => {
                return Err(err);
            },
        };
        
        //asscess_token
        let token = match data.find("component_access_token") {
            Some(token) => token.to_owned(),
            None => { Json::Null}
        };
        //过期秒数
        let expires_in = match data.find("expires_in") {
            Some(expires) => expires.as_u64().unwrap() as usize,
            None => 7200usize,
        };
        let current_timestamp=SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        //过期时间
        let expires_at = current_timestamp + expires_in as i64;
        let token_str = match token {
            Json::String(ref v) => {
                format!("{}", v)
            },
            _ => "".to_string()
        };
        Ok((token_str,expires_at))
    }
    /**
     * 生成预授权码
     */
    pub async fn create_preauthcode(&self,access_token:&str)->WeChatResult<String>{
        let uri=format!("{}{}",WECHAT_URI,format!("/cgi-bin/component/api_create_preauthcode?component_access_token={}",access_token));
        let mut hash=HashMap::new();
        hash.insert("component_appid".to_string(),self.app_id.clone());
        let c=Config::default();
        let api=Client::new(c);
        //post
        let result=api.post(&uri,&hash).await?;
        let data = match api.json_decode(&result) {
            Ok(_data) => _data,
            Err(err) => {
                if let WeChatError::ClientError { errcode, .. } = err {
                    if REFETCH_ACCESS_TOKEN_ERRCODES.contains(&errcode) {
                        self.fetch_access_token().await?;
                        return Err(err);
                    } else {
                        return Err(err);
                    }
                } else {
                    return Err(err);
                }
            },
        };
        
         //asscess_token
         let pre_auth_code = match data.find("pre_auth_code") {
            Some(token) => token.to_owned(),
            None => { Json::Null}
        };
        let pre_auth_code_str = match pre_auth_code {
            Json::String(ref v) => {
                format!("{}", v)
            },
            _ => "".to_string()
        };
        Ok(pre_auth_code_str)
    }
    /**
     * 授权页面
     */
    pub  fn component_login_page(&self,pre_auth_code:&str,redirect_uri:&str,auth_type:u32)-> String{
        use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
        
        let encode_uri= if redirect_uri.starts_with("http") {
            utf8_percent_encode(redirect_uri, NON_ALPHANUMERIC).to_string()
        }
        else{
            utf8_percent_encode(&format!("http://{}",redirect_uri), NON_ALPHANUMERIC).to_string()
        };


        let uri=format!("https://mp.weixin.qq.com/{}",format!("/cgi-bin/componentloginpage?component_appid={}&pre_auth_code={}&auth_type={}&redirect_uri={}",
        self.app_id,pre_auth_code,auth_type,encode_uri));
        uri
    }
}