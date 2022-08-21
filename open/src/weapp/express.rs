//! copyright © ecdata.cn 2022 - present
//! 物流服务
//!
//!

use serde_json::Value;
use wechat_sdk::{json_decode, Client, WechatResult};

use crate::API_DOMAIN;

pub struct Express {
    auth_access_token: String,
}

impl Express {
    /// 创建对象
    pub fn new(auth_access_token: String) -> Self {
        Express {
            auth_access_token: auth_access_token,
        }
    }

    /// 申请开通物流消息
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/logistics-service/applyMsgPlugin.html
    pub async fn apply_open_delivery_msg(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/express/delivery/open_msg/open_openmsg?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
        });

        let res = Client::new().post(&uri, &data).await?;

        json_decode(&res)
    }

    /// 申请开通物流退货组件
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/logistics-service/applyReturnPlugin.html
    pub async fn apply_open_return(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/express/delivery/return/open_return?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
        });

        let res = Client::new().post(&uri, &data).await?;

        json_decode(&res)
    }

    /// 申请开通物流查询组件
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/logistics-service/applyQueryPlugin.html
    pub async fn apply_open_query(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/express/delivery/open_msg/open_query_plugin?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
        });

        let res = Client::new().post(&uri, &data).await?;

        json_decode(&res)
    }
}
