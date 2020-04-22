//! copyright
//! 微信第三方平台基础接口对接

use super::TripartiteConfig;

use super::{get_ticket, Ticket};
use crate::{current_timestamp, Client, WeChatError, WeChatResult};
use serde_json::Value;
use std::collections::HashMap;

// 定义接口请求域名
const API_DOMAIN: &'static str = "https://api.weixin.qq.com";
// 需要刷新AccessToken
const REFETCH_ACCESS_TOKEN_ERRCODES: [i32; 3] = [40001, 40014, 42001];

pub struct Component {
    config: TripartiteConfig,
    ticket: Ticket,
}

impl Component {
    pub fn new(conf: TripartiteConfig) -> Self {
        Component {
            config: conf,
            ticket: get_ticket(),
        }
    }

    /// 获取Aceess Token
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/component_access_token.html
    pub async fn fetch_access_token(&self) -> WeChatResult<(String, i64)> {
        let url = format!("{}{}", API_DOMAIN, "/cgi-bin/component/api_component_token");
        let mut hash = HashMap::new();
        let conf = self.config.clone();
        hash.insert("component_appid".to_string(), conf.app_id);
        hash.insert("component_appsecret".to_string(), conf.secret);
        hash.insert(
            "component_verify_ticket".to_string(),
            self.ticket.access_ticket.clone(),
        );
        let api = Client::new();
        let res= api.post(&url, &hash).await?;
        let data = match api.json_decode(&res) {
            Ok(_data) => _data,
            Err(err) => {
                return Err(err);
            }
        };
        //asscess_token
        let token = match data["component_access_token"].as_str(){
            Some(v)=>v.to_owned(),
            None=>"".to_owned()
        };
        println!("{:?}",data);
        let mut t=self.ticket.clone();
        t.access_token=token.clone();
        t.at_expired_time=current_timestamp() + 7000;
        t.save("");
        Ok((token, t.at_expired_time))
    }

    /// 生成预授权码
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/pre_auth_code.html
    pub async fn create_preauthcode(&self, access_token: &str) -> WeChatResult<String> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/component/api_create_preauthcode?component_access_token={}",
                access_token
            )
        );
        logs!(format!("uri::: {}", uri));

        let conf = self.config.clone();
        let mut hash = HashMap::new();
        hash.insert("component_appid".to_string(), conf.app_id.clone());
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;
        println!("uri::: {:?}", res);
        let data = match api.json_decode(&res) {
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
            }
        };

        //pre_auth_code
        let pre_auth_code = match data["pre_auth_code"].as_str(){
            Some(v)=>v,
            None=>""
        };

        Ok(pre_auth_code.to_owned())
    }

    /// 查询授权
    /// 接口文档地址: https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/authorization_info.html
    pub async fn query_auth(&self, pre_auth_code: &str) -> WeChatResult<serde_json::Value> {
        let conf = self.config.clone();
        let mut t = self.ticket.clone();
        // 获取
        let acc_token = t.get_token(conf.clone()).await;
        let uri = format!(
            "{}/cgi-bin/component/api_query_auth?component_access_token={}",
            API_DOMAIN, acc_token
        );

        // println!("query auth {}", uri);

        let mut hash = HashMap::new();
        hash.insert("component_appid".to_string(), conf.app_id.clone());
        hash.insert("authorization_code".to_string(), pre_auth_code.to_owned());
        //post
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;
        let data = match api.json_decode(&res) {
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
            }
        };
        Ok(data)
        // match api.post(&uri, &hash).await {
        //     Ok(res) => match serde_json::from_str(&res) {
        //         Ok(v) => {
        //             let dic: Value = v;
        //             // println!("auth ::: ==== {:?}", dic);
        //             Ok(dic)
        //         }
        //         Err(_) => Err(WeChatError::InvalidValue),
        //     },
        //     Err(e) => Err(e),
        // }
    }

    /// 获取或者刷新指定小程序或公众号的授权token
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/api_authorizer_token.html
    pub async fn fetch_auth_token(
        &self,
        authorizer_appid: &str,
        refresh_token: &str,
    ) -> WeChatResult<(String, String)> {
        let url = format!(
            "{}/cgi-bin/component/api_authorizer_token?component_access_token={}",
            API_DOMAIN, ""
        );
        let mut hash = HashMap::new();
        let conf = self.config.clone();
        hash.insert("component_appid".to_string(), conf.app_id);
        hash.insert("authorizer_appid".to_string(), authorizer_appid.to_owned());
        hash.insert(
            "authorizer_refresh_token".to_string(),
            refresh_token.to_string(),
        );
        let api = Client::new();
        let res = api.post(&url, &hash).await?;
        let data = match api.json_decode(&res) {
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
            }
        };
        let acc_token = match data["authorizer_access_token"].as_str(){
            Some(v)=>v,
            None=>""
        };
        let ref_token = match data["authorizer_refresh_token"].as_str(){
            Some(v)=>v,
            None=>""
        };
        // let acc_token = data["authorizer_access_token"].to_string();
        // let ref_token = data["authorizer_refresh_token"].to_string();
        Ok((acc_token.to_string(), ref_token.to_string()))

        // match Client::new().post(&url, &hash).await {
        //     Ok(res) => match serde_json::from_str(&res) {
        //         Ok(v) => {
        //             let v: Value = v;
        //             let acc_token = match v["authorizer_access_token"].as_str() {
        //                 Some(v) => v,
        //                 None => "",
        //             };
        //             let ref_token = match v["authorizer_refresh_token"].as_str() {
        //                 Some(v) => v,
        //                 None => "",
        //             };
        //             Ok((acc_token.to_string(), ref_token.to_string()))
        //         }
        //         Err(_) => Err(WeChatError::InvalidValue),
        //     },
        //     Err(e) => Err(e),
        // }
    }

    /// 获取授权信息
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/api_get_authorizer_info.html
    pub async fn fetch_auth_info(&self, _auth_appid: &str) -> WeChatResult<String> {
        Ok("".to_string())
    }

    /// 获取授权方的选项信息
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/api_get_authorizer_option.html
    pub async fn fetch_auth_option(&self, _auth_appid: &str) -> WeChatResult<String> {
        Ok("".to_string())
    }

    /// 设置授权方选项信息
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/api_set_authorizer_option.html
    pub async fn set_auth_option(&self, _auth_appid: &str) -> WeChatResult<String> {
        Ok("".to_string())
    }

    /// 拉取所有已授权的帐号信息
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/api/api_get_authorizer_list.html
    /// returns: (count, vec<appid, refresh_token, auth_time>)
    pub async fn fetch_auth_list(
        &self,
        offset: i64,
        count: i64,
    ) -> WeChatResult<(i64, Vec<(String, String, i64)>)> {
        let conf = self.config.clone();
        let mut ticket = get_ticket();
        let acc_token = ticket.get_token(conf.clone()).await;
        let uri = format!(
            "{}/cgi-bin/component/api_get_authorizer_list?component_access_token={}",
            API_DOMAIN, acc_token
        );
        let mut hash = HashMap::new();
        hash.insert("component_appid".to_string(), conf.app_id.clone());
        hash.insert("offset".to_string(), offset.to_string());
        hash.insert("count".to_string(), count.to_string());

        match Client::new().post(&uri, &hash).await {
            Ok(res) => match serde_json::from_str(&res) {
                Ok(v) => {
                    let v: Value = v;
                    let c = v["total_count"].as_i64().unwrap();
                    let mut list: Vec<(String, String, i64)> = vec![];
                    match v["list"].as_array() {
                        Some(a) => {
                            for x in a {
                                let appid = x["authorizer_appid"].as_str().unwrap();
                                let ref_token = x["refresh_token"].as_str().unwrap();
                                let auth_time = x["auth_time"].as_i64().unwrap();
                                list.push((appid.to_string(), ref_token.to_string(), auth_time))
                            }
                        }
                        None => {}
                    }
                    // println!("{:?}", v);
                    Ok((c, list))
                }
                Err(_) => Err(WeChatError::InvalidValue),
            },
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

        let conf = self.config.clone();
        let uri=format!("https://mp.weixin.qq.com/{}",format!("/cgi-bin/componentloginpage?component_appid={}&pre_auth_code={}&auth_type={}&redirect_uri={}",
        conf.app_id,pre_auth_code,auth_type,encode_uri));
        uri
    }
}
