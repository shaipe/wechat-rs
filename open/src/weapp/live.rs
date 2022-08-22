//! copyright © ecdata.cn 2022 - present
//! 直播服务
//!
//!

use serde_json::Value;
use wechat_sdk::{json_decode, Client, WechatResult};

use crate::API_DOMAIN;

pub struct Live {
    auth_access_token: String,
}

impl Live {
    /// 创建对象
    pub fn new(auth_access_token: &str) -> Self {
        Live {
            auth_access_token: auth_access_token.to_owned(),
        }
    }

    /// 申请开通直播
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/live-player/applyLivelnfo.html
    pub async fn apply_live(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/business/applyliveinfo?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
            "action": "apply"
        });

        let res = Client::new().post(&uri, &data).await?;

        json_decode(&res)
    }
}
