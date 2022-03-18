//! copyright
//! 微信第三方平台基础接口对接


use super::Ticket;
use serde_json::Value;
use std::collections::HashMap;
use wechat_sdk::{current_timestamp, Client, WechatResult};
use super::{TripartiteConfig};
use std::time::{SystemTime, UNIX_EPOCH};
use redis::RedisConfig;
// 定义接口请求域名
const API_DOMAIN: &'static str = "https://api.weixin.qq.com";
// 需要刷新AccessToken
const REFETCH_ACCESS_TOKEN_ERRCODES: [i32; 3] = [40001, 40014, 42001];

pub struct Component {
    tripart_conf: TripartiteConfig,
    redis_conf:RedisConfig,
    redis_con:String
}

impl Component {
    pub fn new(tripart_conf:TripartiteConfig,redis_conf:RedisConfig)->Self{
        let redis_con=format!("redis://{}{}:{}/{}",&redis_conf.password, &redis_conf.server,&redis_conf.port,redis_conf.dbid);
        Component{  
            redis_con:redis_con,
            redis_conf:redis_conf,
            tripart_conf:tripart_conf
           }
    }

    /// 获取Aceess Token
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/component_access_token.html
   
    pub async fn fetch_access_token(&self, access_ticket: String) -> WechatResult<(String, i64)> {
        let url = format!("{}{}", API_DOMAIN, "/cgi-bin/component/api_component_token");
        let mut hash = HashMap::new();
        let conf = self.tripart_conf.clone();
        hash.insert("component_appid".to_string(), conf.app_id);
        hash.insert("component_appsecret".to_string(), conf.secret);
        hash.insert("component_verify_ticket".to_string(), access_ticket);
        let api = Client::new();
        let res = api.post(&url, &hash).await?;
        let data = match wechat_sdk::json_decode(&res) {
            Ok(_data) => _data,
            Err(err) => {
                return Err(err);
            }
        };
        //asscess_token
        let token = match data["component_access_token"].as_str() {
            Some(v) => v.to_owned(),
            None => "".to_owned(),
        };
        println!("{:?}", data);
        let expired_time = current_timestamp() + 7000;
        // let mut t = self.ticket.clone();
        // t.access_token = token.clone();
        // t.at_expired_time = current_timestamp() + 7000;
        // t.save("");
        set_comp_token(&self.redis_con,&self.tripart_conf.app_id ,(token.clone(),expired_time));
        Ok((token, expired_time))
    }
    /// 获取access_token
    pub async fn get_access_token(&self) -> String {
        let (token,_)=self.get_access_tokens().await;
        token
    }
    pub async fn get_access_tokens(&self) -> (String,i64) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let token=match get_comp_token(&self.redis_con, &self.tripart_conf.app_id){
            Ok(s)=>s,
            Err(_)=>{
                ("".to_owned(),0)
            }
        };
        let expires_at: i64 = token.1;
        //比较过期时间
        if expires_at <= timestamp {
            let ticket=Ticket::new(self.tripart_conf.clone(), self.redis_conf.clone());
            let result = self.fetch_access_token(ticket.get_ticket()).await;
           
            match result {
                Ok(token) => token,
                Err(_) => ("".to_owned(),0),
            }
        } else {
            token
        }
    }
    /// 生成预授权码
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/pre_auth_code.html
    /// 
    pub async fn create_preauthcode(&self) -> WechatResult<String> {
        let access_token=self.get_access_token().await;
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/component/api_create_preauthcode?component_access_token={}",
                access_token
            )
        );
        log!("uri::: {}", uri);

        let mut hash = HashMap::new();
        hash.insert("component_appid".to_string(), self.tripart_conf.app_id.clone());
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;
        let data=self.parse_post(&res).await?;
        //pre_auth_code
        match data["pre_auth_code"].as_str() {
            Some(v) => Ok(v.to_owned()),
            None => Err(error!{code:600,msg:"pre_auth_code"}),
        }
    }

    /// 查询授权
    /// 接口文档地址: https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/authorization_info.html
    pub async fn query_auth(&self, pre_auth_code: &str) -> WechatResult<serde_json::Value> {
     
        // 获取
        let acc_token =self.get_access_token().await;
        let uri = format!(
            "{}/cgi-bin/component/api_query_auth?component_access_token={}",
            API_DOMAIN, acc_token
        );

        let mut hash = HashMap::new();
        hash.insert("component_appid".to_string(), self.tripart_conf.app_id.clone());
        hash.insert("authorization_code".to_string(), pre_auth_code.to_owned());
        //post
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;
        self.parse_post(&res).await
       
    }

    /// 获取或者刷新指定小程序或公众号的授权token
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/api_authorizer_token.html
    pub async fn fetch_auth_token(
        &self,
        authorizer_appid: &str,
        refresh_token: &str,
    ) -> WechatResult<(String, String)> {
        let acc_token = self.get_access_token().await;
        let url = format!(
            "{}/cgi-bin/component/api_authorizer_token?component_access_token={}",
            API_DOMAIN, acc_token
        );
        let mut hash = HashMap::new();
        let conf = self.tripart_conf.clone();
        hash.insert("component_appid".to_string(), conf.app_id);
        hash.insert("authorizer_appid".to_string(), authorizer_appid.to_owned());
        hash.insert(
            "authorizer_refresh_token".to_string(),
            refresh_token.to_string(),
        );
        let api = Client::new();
        let res = api.post(&url, &hash).await?;
        let data=self.parse_post(&res).await?;
        let acc_token = match data["authorizer_access_token"].as_str() {
            Some(v) => v,
            None => "",
        };
        let ref_token = match data["authorizer_refresh_token"].as_str() {
            Some(v) => v,
            None => "",
        };
        Ok((acc_token.to_string(), ref_token.to_string()))

    }

    /// 获取授权信息
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/api_get_authorizer_info.html
    pub async fn fetch_auth_info(&self, _auth_appid: &str) -> WechatResult<String> {
        Ok("".to_string())
    }

    /// 获取授权方的选项信息
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/api_get_authorizer_option.html
    pub async fn fetch_auth_option(&self, _auth_appid: &str) -> WechatResult<String> {
        Ok("".to_string())
    }

    /// 设置授权方选项信息
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/api_set_authorizer_option.html
    pub async fn set_auth_option(&self, _auth_appid: &str) -> WechatResult<String> {
        Ok("".to_string())
    }

    /// 拉取所有已授权的帐号信息
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/api_get_authorizer_list.html
    /// returns: (count, vec<appid, refresh_token, auth_time>)
    pub async fn fetch_auth_list(
        &self,
        offset: i64,
        count: i64,
    ) -> WechatResult<(i64, Vec<(String, String, i64)>)> {
        let acc_token = self.get_access_token().await;
        let uri = format!(
            "{}/cgi-bin/component/api_get_authorizer_list?component_access_token={}",
            API_DOMAIN, acc_token
        );
        let mut hash = HashMap::new();
        hash.insert("component_appid".to_string(), self.tripart_conf.app_id.clone());
        hash.insert("offset".to_string(), offset.to_string());
        hash.insert("count".to_string(), count.to_string());
        let res=Client::new().post(&uri, &hash).await? ;
        let data=self.parse_post(&res).await?;
       
        let c = data["total_count"].as_i64().unwrap();
        let mut list: Vec<(String, String, i64)> = vec![];
        match data["list"].as_array() {
            Some(a) => {
                for x in a {
                    let appid = x["authorizer_appid"].as_str().unwrap();
                    let ref_token = x["refresh_token"].as_str().unwrap();
                    let auth_time = x["auth_time"].as_i64().unwrap();
                    list.push((appid.to_string(), ref_token.to_string(), auth_time))
                }
                Ok((c,list))
            }
            None => {
                Err(error!{code:600,msg:"error"})
            }
        }

    }

    /// 授权页面
    pub fn component_login_page(
        &self,
        pre_auth_code: &str,
        redirect_uri: &str,
        auth_type: u32,
    ) -> String {
        use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

        let encode_uri = if redirect_uri.starts_with("http") {
            utf8_percent_encode(redirect_uri, NON_ALPHANUMERIC).to_string()
        } else {
            utf8_percent_encode(&format!("http://{}", redirect_uri), NON_ALPHANUMERIC).to_string()
        };

        let conf = self.tripart_conf.clone();
        let uri=format!("https://mp.weixin.qq.com/{}",format!("/cgi-bin/componentloginpage?component_appid={}&pre_auth_code={}&auth_type={}&redirect_uri={}",
        conf.app_id,pre_auth_code,auth_type,encode_uri));
        uri
    }
    //获取模版列表
    pub async fn get_template_list(&self, template_type: u32) -> WechatResult<Vec<serde_json::Value>> {
     
        // 获取
        let acc_token =self.get_access_token().await;
        let uri = format!(
            "{}/wxa/gettemplatelist?access_token={}",
            API_DOMAIN, acc_token
        );

        let mut hash = HashMap::new();
        hash.insert("template_type".to_string(), template_type);
        //post
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;
        let data=self.parse_post(&res).await?;
        let list_temp=data["template_list"].as_array().unwrap();
        let mut list:Vec<Value>=vec![];
        for a in list_temp{
            let mut v: serde_json::map::Map<
                std::string::String,
                serde_json::value::Value,
            > = serde_json::map::Map::new();
            let template_id=a["template_id"].as_str().unwrap();
            let user_version=a["user_version"].as_str().unwrap();
            v.insert("template_id".to_owned(), Value::String(template_id.to_string()));
            v.insert("user_version".to_owned(), Value::String(user_version.to_string()));
            list.push(serde_json::to_value(v).unwrap());
        }
        Ok(list)
    }

    //解析post请求结果
    pub async fn parse_post(&self,res:&str)->WechatResult<Value>{
        let data = match wechat_sdk::json_decode(&res) {
            Ok(_data) => _data,
            Err(err) => {
                use wechat_sdk:: ErrorKind;
                if let ErrorKind::Custom { code, .. } = err.kind {
                    if REFETCH_ACCESS_TOKEN_ERRCODES.contains(&code) {
                        self.get_access_token().await;
                        return Err(err);
                    } else {
                        return Err(err);
                    }
            } else {
                return Err(err);
            }}
        };
        Ok(data)
    }
}
const COMP_CATCHE_KEY: &str = "COMP_CATCHE_KEY_";

use crate::redis::{RedisStorage, SessionStore};


/// 设置单个
pub fn set_comp_token(redis_con: &str, key: &str, cnf: (String, i64)) {
    let url = format!("{}", redis_con);
    let cache_key = format!(
        "{0}_{1}",
        COMP_CATCHE_KEY,
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
pub fn get_comp_token(redis_con: &str, key: &str) -> WechatResult<(String, i64)> {
    let cache_key = format!(
        "{0}_{1}",
        COMP_CATCHE_KEY,
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