//! copyright © ecdata.cn 2021 - present
//! 小程序代码管理

use wechat_sdk::{Client, WechatResult,get_url_encode};

use crate::min_category::MinCategoryItem;

const API_DOMAIN: &'static str = "https://api.weixin.qq.com";

pub struct MinCode {
    authorizer_access_token: String,
}
impl MinCode {
    pub fn new(_authorizer_access_token: &str) -> Self {
        MinCode {
            authorizer_access_token: _authorizer_access_token.to_string(),
        }
    }
    /// 配置小程序用户隐私保护指引
    pub async fn set_privacy(&self, data: serde_json::Value) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/component/setprivacysetting?access_token={}",
                self.authorizer_access_token.clone()
            )
        );

        let api = Client::new();
        let res = api.post(&uri, &data).await?;

        wechat_sdk::json_decode(&res)
    }
    /// 上传代码
    pub async fn commit_code(
        &self,
        template_id: &str,
        ext_json: serde_json::Value,
        user_version: &str,
        user_desc: &str,
    ) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/commit?access_token={}",
                self.authorizer_access_token.clone()
            )
        );

        let data = json!(
        {
            "template_id":template_id,
            "ext_json": ext_json.to_string(),
            "user_version":user_version,
            "user_desc":user_desc
        });

        let api = Client::new();
        let res = api.post(&uri, &data).await?;

        wechat_sdk::json_decode(&res)
    }

    /// 获取体验版二维码
    pub async fn get_qrcode(&self, _path: &str) -> WechatResult<Vec<u8>> {
        let path = get_url_encode(_path);

        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/get_qrcode?access_token={}&path={}",
                self.authorizer_access_token.clone(),
                path
            )
        );
        println!("path={}", uri);
        let api = Client::new();
        api.get_bytes(&uri).await
    }

    /// 提交审核
    pub async fn submit_audit(&self, item: MinCategoryItem) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/submit_audit?access_token={}",
                self.authorizer_access_token.clone()
            )
        );
        let data = json!({ "item_list": [item] });
        let api = Client::new();
        let res = api.post(&uri, &data).await?;
        wechat_sdk::json_decode(&res)
    }

    /// 查询指定版本的审核状态
    pub async fn audit_status(&self, auditid: i64) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/get_auditstatus?access_token={}",
                self.authorizer_access_token.clone()
            )
        );
        let data = json!({ "auditid": auditid });
        let api = Client::new();
        let res = api.post(&uri, &data).await?;
        wechat_sdk::json_decode(&res)
    }

    /// 查询指定版本的审核状态
    pub async fn latest_audit_status(&self) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/get_latest_auditstatus?access_token={}",
                self.authorizer_access_token.clone()
            )
        );
        let api = Client::new();
        let res = api.get(&uri).await?;
        wechat_sdk::json_decode(&res)
    }
    /// 小程序审核撤回
    pub async fn undo_audit(&self) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/undocodeaudit?access_token={}",
                self.authorizer_access_token.clone()
            )
        );
        let api = Client::new();
        let res = api.get(&uri).await?;
        wechat_sdk::json_decode(&res)
    }
    /// 发布已审核通过的小程序
    pub async fn release(&self) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/release?access_token={}",
                self.authorizer_access_token.clone()
            )
        );
        let data = json!({});
        let api = Client::new();
        let res = api.post(&uri, &data).await?;
        wechat_sdk::json_decode(&res)
    }
    /// 版本回退
    pub async fn revert_code(&self) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/revertcoderelease?access_token={}",
                self.authorizer_access_token.clone()
            )
        );
        let api = Client::new();
        let res = api.get(&uri).await?;
        wechat_sdk::json_decode(&res)
    }

    /// 加急审核申请
    pub async fn speed_up_audit(&self, auditid: i64) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/speedupaudit?access_token={}",
                self.authorizer_access_token.clone()
            )
        );
        let data = json!({ "auditid": auditid });
        let api = Client::new();
        let res = api.post(&uri, &data).await?;
        wechat_sdk::json_decode(&res)
    }

    /// 获取小程序码
    pub async fn get_wxa_code(
        &self,
        path: &str,
        width: u32,
        auto_color: bool,
        line_color: &str,
        is_hyaline: bool,
    ) -> WechatResult<Vec<u8>> {
        let path = get_url_encode(path);
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/getwxacode?access_token={}",
                self.authorizer_access_token.clone()
            )
        );
        let mut data = json!({
        "path": &path,
        "width": width,
        "auto_color": auto_color,
        "is_hyaline": is_hyaline,
        });
        if auto_color{
            data["line_color"]=serde_json::Value::String(line_color.to_owned());
        }
        let api = Client::new();
        api.request_betyes("get", &uri, &data).await

    }
}
