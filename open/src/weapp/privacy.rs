//! copyright © ecdata.cn 2022 - present
//! 配置小程序用户隐私保护
//!

use wechat_sdk::{Client, WechatResult};
use crate::API_DOMAIN;

pub struct Privacy {
    // 第三方平台接口调用令牌
    auth_access_token: String,
}

impl Privacy {
    /// 创建隐私设置对象
    /// param1: 第三方平台接口调用令牌
    pub fn new(access_token: &str) -> Self {
        Privacy {
            auth_access_token: access_token.to_string(),
        }
    }

    /// 配置小程序用户隐私保护指引
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/privacy_config/set_privacy_setting.html
    pub async fn set_privacy(&self, data: serde_json::Value) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/component/setprivacysetting?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let res = Client::new().post(&uri, &data).await?;
        wechat_sdk::json_decode(&res)
    }

    // 查询小程序用户隐私保护指引
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/privacy_config/get_privacy_setting.html


    // 上传小程序用户隐私保护指引
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/privacy_config/upload_privacy_exfile.html

    // 获取隐私接口列表
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/apply_api/get_privacy_interface.html

    // 申请隐私接口
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/apply_api/apply_privacy_interface.html

    
}
