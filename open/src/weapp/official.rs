//! copyright © ecdata.cn 2022 - present
//! 扫码关注公众号
//! 

use serde_json::Value;
use wechat_sdk::{WechatResult, Client, json_decode};

use crate::API_DOMAIN;


pub struct Official {
    auth_access_token: String
}

impl Official {

    /// 创建对象
    pub fn new(auth_access_token: String) -> Self {
        Official { auth_access_token: auth_access_token}
    }

    /// 获取已设置公众号信息
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/subscribe-component/getShowItem.html
    pub async fn get_officials(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/getshowwxaitem?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let res = Client::new().get(&uri).await?;

        json_decode(&res)
    }

    /// 获取可设置公众号列表
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/subscribe-component/getLinkForShow.html
    pub async fn get_enable_officials(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/getwxamplinkforshow?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let res = Client::new().get(&uri).await?;

        json_decode(&res)
    }

    /// 设置扫码关注的公众号
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/subscribe-component/setShowItem.html
    /// @param1-biz_flag: 是否打开扫码关注组件，0 关闭，1 开启
    /// @param1 - appid: 如果开启，需要传新的公众号 appid
    pub async fn set_scan_subscribe(&self, biz_flag: i8, appid: &str) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/updateshowwxaitem?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
            "wxa_subscribe_biz_flag": biz_flag,
            "appid": appid
        });


        let res = Client::new().post(&uri, &data).await?;

        json_decode(&res)
    }
}