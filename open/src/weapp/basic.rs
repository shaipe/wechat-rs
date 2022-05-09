//! copyright © ecdata.cn 2021 - present
//! 小程序域名设置

use wechat_sdk::{Client, WechatResult};

const API_DOMAIN: &'static str = "https://api.weixin.qq.com";

pub struct Basic {
    auth_access_token: String,
}
impl Basic {
    pub fn new(_auth_access_token: &str) -> Self {
        Basic {
            auth_access_token: _auth_access_token.to_string(),
        }
    }

    // 获取基本信息
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Mini_Program_Basic_Info/Mini_Program_Information_Settings.html

    // 查询公众号/小程序是否绑定open帐号
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Mini_Program_Basic_Info/getbindopeninfo.html

    /// 设置服务器域名
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Mini_Program_Basic_Info/Server_Address_Configuration.html
    pub async fn set_server_domain(
        &self,
        req_domain: Vec<String>,
        ws_domain: Vec<String>,
        upload_domain: Vec<String>,
        down_domain: Vec<String>,
    ) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/modify_domain?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!({
            "action":"set",
            "requestdomain":req_domain,
            "wsrequestdomain":ws_domain,
            "uploaddomain":upload_domain,
            "downloaddomain":down_domain,
        });

        let res = Client::new().post(&uri, &data).await?;
        wechat_sdk::json_decode(&res)
    }

    /// 设置业务域名
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Mini_Program_Basic_Info/setwebviewdomain.html
    pub async fn set_webview_domain(
        &self,
        req_domain: Vec<String>,
    ) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/setwebviewdomain?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let data = json!({
            "action":"set",
            "webviewdomain":req_domain
        });

        let res = Client::new().post(&uri, &data).await?;
        wechat_sdk::json_decode(&res)
    }

    // 设置名称
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Mini_Program_Basic_Info/setnickname.html

    // 微信认证名称检测
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Mini_Program_Basic_Info/wxverify_checknickname.html

    // 名称审核结果事件推送接收
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Mini_Program_Basic_Info/wxa_nickname_audit.html

    // 查询改名审核状态
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Mini_Program_Basic_Info/api_wxa_querynickname.html

    // 修改头像
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Mini_Program_Basic_Info/modifyheadimage.html

    // 修改功能介绍
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Mini_Program_Basic_Info/modifysignature.html
}
