//! copyright © ecdata.cn 2022 - present
//! 微信公众号授权

// use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use wechat_sdk::{Client, WechatResult};

use crate::API_DOMAIN;

// const WECHAT_OPEN_URI: &'static str = "https://open.weixin.qq.com";
// const API_DOMAIN: &'static str = "https://api.weixin.qq.com";

/// 开放平台号
pub struct OpenAccount {
    app_id: String,
    auth_access_token: String,
}

impl OpenAccount {
    /// 创建开放平台号
    pub fn new(app_id: &str, access_token: &str) -> Self {
        OpenAccount {
            app_id: app_id.to_string(),
            auth_access_token: access_token.to_string(),
        }
    }

    /// 创建开放平台帐号并绑定公众号或小程序
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/account/create.html
    pub async fn create_open(&self) -> WechatResult<String> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/open/create?access_token={}",
                self.auth_access_token.clone()
            )
        );
        // log!("uri::: {},{}", uri, self.app_id);

        let mut hash = HashMap::new();
        hash.insert("appid".to_string(), self.app_id.clone());
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;

        let data = wechat_sdk::json_decode(&res)?;
        // println!("res-------={:?}", res);
        let open_appid = match data["open_appid"].as_str() {
            Some(v) => v,
            None => "",
        };
        Ok(open_appid.to_owned())
    }

    /// 将公众号或小程序绑定到开放平台帐号
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/account/bind.html
    pub async fn bind_open(&self, open_app_id: &str) -> WechatResult<bool> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/open/bind?access_token={}",
                self.auth_access_token.clone()
            )
        );
        // log!("uri::: {}", uri);

        let mut hash = HashMap::new();
        hash.insert("appid".to_string(), self.app_id.clone());
        hash.insert("open_appid".to_string(), open_app_id.to_owned());
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;
        let mut bo = false;
        if wechat_sdk::json_decode(&res).is_ok() {
            bo = true;
        }
        Ok(bo)
    }

    /// 将公众号或小程序从开放平台帐号中解绑
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/account/unbind.html
    pub async fn unbind_open(&self, open_app_id: &str) -> WechatResult<bool> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/open/unbind?access_token={}",
                self.auth_access_token.clone()
            )
        );
        // log!("uri::: {}", uri);

        let mut hash = HashMap::new();
        hash.insert("appid".to_string(), self.app_id.clone());
        hash.insert("open_appid".to_string(), open_app_id.to_owned());
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;
        let mut bo = false;
        if wechat_sdk::json_decode(&res).is_ok() {
            bo = true;
        }
        Ok(bo)
    }

    /// 获取公众号/小程序所绑定的开放平台帐号
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/account/get.html
    pub async fn get_open_account(&self) -> WechatResult<String> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/open/get?access_token={}",
                self.auth_access_token.clone()
            )
        );
        // log!("uri::: {}", uri);

        let mut hash = HashMap::new();
        hash.insert("appid".to_string(), self.app_id.clone());
        let api = Client::new();
        let res = api.post(&uri, &hash).await?;
        let data = wechat_sdk::json_decode(&res)?;
        Ok(data["open_appid"].to_string())
    }
}
