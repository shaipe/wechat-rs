//! copyright © ecdata.cn 2021 - present
//! 微信公众号授权

use wechat_sdk::{Client, WechatResult};
use std::time::{SystemTime, UNIX_EPOCH};
use redis::RedisConfig;
use std::collections::HashMap;
use serde_json::Value;
use crate::redis::{RedisStorage, SessionStore};
const WECHAT_OPEN_URI: &'static str = "https://open.weixin.qq.com";
const API_DOMAIN: &'static str ="https://api.weixin.qq.com";

///网页授权
pub struct WechatAuthorize {
    app_id: String,
    com_app_id: String,
}
impl WechatAuthorize {
    ///
    pub fn new(_app_id: &str, _com_app_id: &str, _com_access_token: &str) -> WechatAuthorize {
        WechatAuthorize {
            app_id: _app_id.to_string(),
            com_app_id: _com_app_id.to_string(),
        }
    }
    //创建开放平台帐号并绑定公众号或小程序
    pub async fn create_open(&self,access_token:&str)->WechatResult<String>{
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/open/create?access_token={}",
                access_token
            )
        );
        log!("uri::: {}", uri);

        let mut hash = HashMap::new();
        hash.insert("appid".to_string(), self.app_id.clone());
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;
        let data=match wechat_sdk::json_decode(&res) {
            Ok(_data) => _data,
            Err(err) => {
                return Err(err);
             }
        };
        let open_appid = match data["open_appid"].as_str() {
            Some(v) => v,
            None => "",
        };
        Ok(open_appid.to_owned())
    }
    //将公众号或小程序绑定到开放平台帐号
    pub async fn bind_open(&self,access_token:&str,open_app_id:&str)->WechatResult<bool>{
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/open/bind?access_token={}",
                access_token
            )
        );
        log!("uri::: {}", uri);

        let mut hash = HashMap::new();
        hash.insert("appid".to_string(), self.app_id.clone());
        hash.insert("open_appid".to_string(), open_app_id.to_owned());
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;
        let data=match wechat_sdk::json_decode(&res) {
            Ok(_data) => _data,
            Err(err) => {
                return Err(err);
             }
        };
        let bo=match data["errcode"].as_u64() {
            Some(v) => v==0,
            None => false,
        };
        Ok(bo)
    }
      //将公众号或小程序从开放平台帐号中解绑
      pub async fn unbind_open(&self,access_token:&str,open_app_id:&str)->WechatResult<bool>{
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/open/unbind?access_token={}",
                access_token
            )
        );
        log!("uri::: {}", uri);

        let mut hash = HashMap::new();
        hash.insert("appid".to_string(), self.app_id.clone());
        hash.insert("open_appid".to_string(), open_app_id.to_owned());
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;
        let data=match wechat_sdk::json_decode(&res) {
            Ok(_data) => _data,
            Err(err) => {
                return Err(err);
             }
        };
        let bo=match data["errcode"].as_u64() {
            Some(v) => v==0,
            None => false,
        };
        Ok(bo)
    }
    /// 授权页面
    pub fn get_authorize_url(
        &self,
        redirect_uri: &str,
        state: &str,
        scope: &Vec<&str>,
        response_type: &str,
    ) -> String {
        use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

        let encode_uri = if redirect_uri.starts_with("http") {
            utf8_percent_encode(redirect_uri, NON_ALPHANUMERIC).to_string()
        } else {
            utf8_percent_encode(&format!("http://{}", redirect_uri), NON_ALPHANUMERIC).to_string()
        };

        let uri=format!("{}{}",WECHAT_OPEN_URI,format!("/connect/oauth2/authorize?appid={}&redirect_uri={}&response_type={}&scope={}&state={}&component_appid={}#wechat_redirect",
        self.app_id,encode_uri,response_type,scope.join(","),state,self.com_app_id));

        println!("authorize url: {}", uri);
        
        uri
    }
}

const AUTHORIZE_CATCHE_KEY: &str = "AUTHORIZE_CATCHE_KEY";

/// 设置单个
pub fn set_authorize_token(redis_con: &str, key: &str, cnf: (String, i64)) {
    let url = format!("{}", redis_con);
    let cache_key = format!(
        "{0}_{1}",
        AUTHORIZE_CATCHE_KEY,
        key
    );
    match RedisStorage::from_url(url) {
        Ok(session) => {
            session.set(cache_key, format!("{}_{}",cnf.0,cnf.1),Some(2*55*60));
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
/// 获取
pub fn get_authorize_token(redis_con: &str, key: &str) -> WechatResult<(String, i64)> {
    let cache_key = format!(
        "{0}_{1}",
        AUTHORIZE_CATCHE_KEY,
        key
    );
    match RedisStorage::from_url(format!("{}", redis_con)) {
        Ok(session) => {
            let d="".to_owned();
            if let Some(v) = session.get(cache_key, "get".to_owned(), Some(d)) {
                let arr:Vec<_>=v.split('_').collect();
                if arr.len()==2{
                    
                    return Ok((arr[0].to_string(), arr[1].parse::<i64>().unwrap()));
                }
                Err(error!{code:600,msg:"数据不准确"})
            } else {
                Err(error!{code:600,msg:"数据不准确"})
            }
        }
        Err(e) => {
            let msg=format!("{:?}", e);
            Err(error!{code:600,msg:msg})
        }
    }
}