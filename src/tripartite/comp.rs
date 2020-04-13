use super::TripartiteConfig;

use super::{get_ticket, Ticket};
use crate::{current_timestamp, Client, WeChatError, WeChatResult};
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

    /// 生成预授权码
    pub async fn create_preauthcode(&self, access_token: &str) -> WeChatResult<String> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/component/api_create_preauthcode?component_access_token={}",
                access_token
            )
        );
        let conf = self.config.clone();
        let mut hash = HashMap::new();
        hash.insert("component_appid".to_string(), conf.app_id.clone());

        match Client::new().post(&uri, &hash).await {
            Ok(res) => {
                let v = json!(&res);
                if let Some(code) = v["pre_auth_code"].as_str() {
                    Ok(code.to_string())
                } else {
                    Err(WeChatError::InvalidValue)
                }
            }
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

    /// 获取Aceess Token
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

        match Client::new().post(&url, &hash).await {
            Ok(res) => {
                let v = json!(&res);
                if let Some(token) = v["component_access_token"].as_str() {
                    let expires_at: i64 = current_timestamp() + 7000;
                    Ok((token.to_string(), expires_at))
                } else {
                    Err(WeChatError::InvalidValue)
                }
            }
            Err(err) => Err(err),
        }
    }

    // pub async fn refresh_access_token(&self, ) -> WeChatResult<(String, i64)> {
    //     let url = format!("{}/cgi-bin/component/api_authorizer_token?component_access_token={}")
    // }

    /// 查询授权
    pub async fn query_auth(&self, pre_auth_code: &str) -> WeChatResult<serde_json::Value> {
        let conf = self.config.clone();
        let mut t = self.ticket.clone();
        // 获取
        let acc_token = t.get_token(conf.clone()).await;
        let uri = format!(
            "{}/cgi-bin/component/api_query_auth?component_access_token={}",
            API_DOMAIN, acc_token
        );

        let mut hash = HashMap::new();
        hash.insert("component_appid".to_string(), conf.app_id.clone());
        hash.insert("authorization_code".to_string(), pre_auth_code.to_owned());
        //post
        match Client::new().post(&uri, &hash).await {
            Ok(res) => Ok(json!(&res)),
            Err(e) => Err(e),
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
