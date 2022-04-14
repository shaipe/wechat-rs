//! copyright © ecdata.cn 2021 - present
//! 微信第三方平台授权与Token
//!

use crate::API_DOMAIN;

use super::Config;
use serde_json::Value;
use std::collections::HashMap;
use wechat_sdk::{current_timestamp, Client, WechatResult};

///
pub struct AuthToken {
    tripart_conf: Config,
}

impl AuthToken {
    /// Create a new instance
    pub fn new(tripart_conf: Config) -> Self {
        AuthToken {
            tripart_conf: tripart_conf,
        }
    }

    /// 获取令牌
    /// 获取Aceess Token
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/component_access_token.html
    pub async fn fetch_access_token(&self, access_ticket: String) -> WechatResult<(String, u64)> {
        let url = format!("{}{}", API_DOMAIN, "/cgi-bin/component/api_component_token");
        let mut hash = HashMap::new();
        let conf = self.tripart_conf.clone();
        hash.insert("component_appid".to_string(), conf.app_id);
        hash.insert("component_appsecret".to_string(), conf.secret);
        hash.insert("component_verify_ticket".to_string(), access_ticket.clone());
        let api = Client::new();
        let res = api.post(&url, &hash).await?;

        let data = match wechat_sdk::json_decode(&res) {
            Ok(_data) => _data,
            Err(err) => {
                return Err(err);
            }
        };

        // asscess_token
        let token = match data["component_access_token"].as_str() {
            Some(v) => v.to_owned(),
            None => "".to_owned(),
        };
        // println!("component_access_token={:?}", data);
        let expired_time = current_timestamp() + 7000;
        //set_comp_token(&self.redis_con,&self.tripart_conf.app_id ,(token.clone(),expired_time));
        Ok((token, expired_time))
    }

    /// 获取预授权码
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/pre_auth_code.html
    pub async fn create_preauthcode(&self, access_token: &str) -> WechatResult<String> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/component/api_create_preauthcode?component_access_token={}",
                access_token
            )
        );
        // log!("uri::: {}", uri);

        let mut hash = HashMap::new();
        hash.insert(
            "component_appid".to_string(),
            self.tripart_conf.app_id.clone(),
        );
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;
        let data = crate::parse_json(&res).await?;
        //pre_auth_code
        match data["pre_auth_code"].as_str() {
            Some(v) => Ok(v.to_owned()),
            None => Err(error! {code:600,msg:"pre_auth_code"}),
        }
    }

    /// 使用授权码获取授权信息
    /// 接口文档地址: https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/authorization_info.html
    pub async fn query_auth(
        &self,
        auth_code: &str,
        acc_token: &str,
    ) -> WechatResult<serde_json::Value> {
        // 获取
        let uri = format!(
            "{}/cgi-bin/component/api_query_auth?component_access_token={}",
            API_DOMAIN, acc_token
        );

        let mut hash = HashMap::new();
        hash.insert(
            "component_appid".to_string(),
            self.tripart_conf.app_id.clone(),
        );
        hash.insert("authorization_code".to_string(), auth_code.to_owned());
        //post
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;
        crate::parse_json(&res).await
    }

    /// 获取/刷新接口调用令牌
    /// 获取或者刷新指定小程序或公众号的调用令牌
    /// POST https://api.weixin.qq.com/cgi-bin/component/api_authorizer_token?component_access_token=COMPONENT_ACCESS_TOKEN
    pub async fn fetch_authorizer_token(
        &self,
        authorizer_appid: &str,
        refresh_token: &str,
        comp_access_token: &str,
    ) -> WechatResult<(String, u64)> {
        let url = format!(
            "{}/cgi-bin/component/api_authorizer_token?component_access_token={}",
            API_DOMAIN, comp_access_token
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
        let data = crate::parse_json(&res).await?;
        let acc_token = match data["auth_access_token"].as_str() {
            Some(v) => v,
            None => "",
        };
        // let ref_token = match data["authorizer_refresh_token"].as_str() {
        //     Some(v) => v,
        //     None => "",
        // };
        let expired_time = current_timestamp() + 7000;
        Ok((acc_token.to_string(), expired_time))
    }

    /// 获取授权帐号详情
    /// POST https://api.weixin.qq.com/cgi-bin/component/api_get_authorizer_info?component_access_token=COMPONENT_ACCESS_TOKEN
    pub async fn fetch_authorizer_info(
        &self,
        authorizer_appid: &str,
        comp_access_token: &str,
    ) -> WechatResult<Value> {
        let url = format!(
            "{}/cgi-bin/component/api_get_authorizer_info?component_access_token={}",
            API_DOMAIN, comp_access_token
        );
        let mut hash = HashMap::new();
        let conf = self.tripart_conf.clone();
        hash.insert("component_appid".to_string(), conf.app_id);
        hash.insert("authorizer_appid".to_string(), authorizer_appid.to_owned());
        hash.insert(
            "component_appid".to_owned(),
            self.tripart_conf.app_id.clone(),
        );

        let res = Client::new().post(&url, &hash).await?;
        log!("==== {}", res);
        let data = crate::parse_json(&res).await?;
        Ok(data)
    }

    /// 拉取所有已授权的帐号信息
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/api_get_authorizer_list.html
    /// returns: (count, vec<appid, refresh_token, auth_time>)
    pub async fn fetch_authorizer_list(
        &self,
        offset: i64,
        count: i64,
        comp_access_token: &str,
    ) -> WechatResult<(i64, Vec<(String, String, i64)>)> {
        let uri = format!(
            "{}/cgi-bin/component/api_get_authorizer_list?component_access_token={}",
            API_DOMAIN, comp_access_token
        );
        let mut hash = HashMap::new();
        hash.insert(
            "component_appid".to_string(),
            self.tripart_conf.app_id.clone(),
        );
        hash.insert("offset".to_string(), offset.to_string());
        hash.insert("count".to_string(), count.to_string());
        let res = Client::new().post(&uri, &hash).await?;
        let data = crate::parse_json(&res).await?;

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
                Ok((c, list))
            }
            None => Err(error! {code:600,msg:"error"}),
        }
    }

    // 获取授权方选项信息
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/ThirdParty/Account_Authorization/api_get_authorizer_option.html

    // 设置授权方选项信息
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/ThirdParty/Account_Authorization/api_set_authorizer_option.html

    /// 授权页面
    pub fn component_login_page(
        &self,
        pre_auth_code: &str,
        redirect_uri: &str,
        auth_type: &str,
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
}
