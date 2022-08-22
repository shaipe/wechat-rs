//! copyright © ecdata.cn 2022 - present
//! 配置小程序用户隐私保护
//!

use crate::API_DOMAIN;
use serde_json::Value;
use wechat_sdk::{json_decode, Client, WechatResult};

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
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/privacy-management/setPrivacySetting.html
    pub async fn set_privacy(&self, data: Value) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/component/setprivacysetting?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 查询小程序用户隐私保护指引
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/privacy-management/getPrivacySetting.html
    /// privacy_ver	number	是	1表示现网版本，即，传1则该接口返回的内容是现网版本的；2表示开发版，即，传2则该接口返回的内容是开发版本的。默认是2。
    pub async fn get_privacy(&self, privacy_ver: i32) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/component/setprivacysetting?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!({ "privacy_ver": privacy_ver });

        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 上传小程序用户隐私保护指引 TODO:
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/privacy_config/upload_privacy_exfile.html
    /// file	bufffer	是	只支持传 txt 文件
    pub async fn upload_user_privacy(&self, privacy_ver: i32) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/component/uploadprivacyextfile?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!({ "privacy_ver": privacy_ver });

        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 申请位置
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/privacy-api-management/applyPrivacyInterface.html
    /// api_name	string	是	申请的 api 英文名，例如wx.choosePoi，严格区分大小写
    /// content	string	是	申请说原因，不超过300个字符；需要以utf-8编码提交，否则会出现审核失败
    /// url_list	array<string>	否	(辅助网页)例如，上传官网网页链接用于辅助审核
    /// pic_list	array<string>	否	(辅助图片)填写图片的url ，最多10个
    /// video_list	array<string>	否	(辅助视频)填写视频的链接 ，最多支持1个；视频格式只支持mp4格式
    pub async fn apply_privacy_interface(
        &self,
        api_name: &str,
        content: &str,
        url_list: Vec<String>,
        pic_list: Vec<String>,
        video_list: Vec<String>,
    ) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/security/apply_privacy_interface?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!({
            "api_name": api_name,
            "content": content,
            "pic_list": url_list,
            "video_list":pic_list,
            "url_list":video_list
        });

        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 获取安全接口列表
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/privacy-api-management/getPrivacyInterface.html
    pub async fn get_privacy_interface(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/security/get_privacy_interface?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let res = Client::new().get(&uri).await?;
        json_decode(&res)
    }
}
