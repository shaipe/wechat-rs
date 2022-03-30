//! copyright © ecdata.cn 2021 - present
//! 微信公众号授权

use std::collections::HashMap;
use wechat_redis::{RedisStorage, SessionStore};
use wechat_sdk::{Client, WechatResult};
const WECHAT_OPEN_URI: &'static str = "https://open.weixin.qq.com";
const API_DOMAIN: &'static str = "https://api.weixin.qq.com";

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
    let cache_key = format!("{0}_{1}", AUTHORIZE_CATCHE_KEY, key);
    match RedisStorage::from_url(url) {
        Ok(session) => {
            session.set(cache_key, format!("{}_{}", cnf.0, cnf.1), Some(2 * 55 * 60));
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
/// 获取
pub fn get_authorize_token(redis_con: &str, key: &str) -> WechatResult<(String, i64)> {
    let cache_key = format!("{0}_{1}", AUTHORIZE_CATCHE_KEY, key);
    match RedisStorage::from_url(format!("{}", redis_con)) {
        Ok(session) => {
            let d = "".to_owned();
            if let Some(v) = session.get(cache_key, "get".to_owned(), Some(d)) {
                let arr: Vec<_> = v.split('_').collect();
                if arr.len() == 2 {
                    return Ok((arr[0].to_string(), arr[1].parse::<i64>().unwrap()));
                }
                Err(error! {code:600,msg:"数据不准确"})
            } else {
                Err(error! {code:600,msg:"数据不准确"})
            }
        }
        Err(e) => {
            let msg = format!("{:?}", e);
            Err(error! {code:600,msg:msg})
        }
    }
}
