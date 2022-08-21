//! copyright © ecdata.cn 2021 - present
//! 小程序代码管理

use serde_json::Value;
use wechat_sdk::{get_url_encode, json_decode, Client, WechatResult};

use super::category::CategoryItem;

const API_DOMAIN: &'static str = "https://api.weixin.qq.com";

pub struct Code {
    auth_access_token: String,
}
impl Code {
    pub fn new(_auth_access_token: &str) -> Self {
        Code {
            auth_access_token: _auth_access_token.to_string(),
        }
    }

    /// 上传代码并生成体验版
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/commit.html
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
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
            "template_id":template_id,
            "ext_json": ext_json.to_string(),
            "user_version":user_version,
            "user_desc":user_desc
        });

        let res = Client::new().post(&uri, &data).await?;

        wechat_sdk::json_decode(&res)
    }

    /// 获取已上传的代码页面列表
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/getCodePage.html
    pub async fn get_pages(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/get_page?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let res = Client::new().get(&uri).await?;
        json_decode(&res)
    }

    /// 获取体验版二维码
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/getTrialQRCode.html
    pub async fn get_qrcode(&self, _path: &str) -> WechatResult<Vec<u8>> {
        let path = get_url_encode(_path);

        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/get_qrcode?access_token={}&path={}",
                self.auth_access_token.clone(),
                path
            )
        );
        println!("path={}", uri);
        Client::new().get_bytes(&uri).await
    }

    /// 提交代码审核
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/submitAudit.html
    pub async fn submit_audit(&self, item: CategoryItem) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/submit_audit?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let data = json!({ "item_list": [item] });
        println!("data=={:?}", data);
        let res = Client::new().post(&uri, &data).await?;
        wechat_sdk::json_decode(&res)
    }

    /// 查询审核单状态
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/getAuditStatus.html
    pub async fn audit_status(&self, auditid: i64) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/get_auditstatus?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let data = json!({ "auditid": auditid });
        let res = Client::new().post(&uri, &data).await?;
        wechat_sdk::json_decode(&res)
    }


    /// 撤回代码审核
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/undoAudit.html
    pub async fn undo_audit(&self) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/undocodeaudit?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let res = Client::new().get(&uri).await?;
        wechat_sdk::json_decode(&res)
    }

    /// 发布已通过审核的小程序
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/release.html
    pub async fn release(&self) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/release?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let data = json!({});

        let res = Client::new().post(&uri, &data).await?;
        wechat_sdk::json_decode(&res)
    }

    /// 小程序版本回退
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/revertCodeRelease.html
    pub async fn revert_code(&self) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/revertcoderelease?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let res = Client::new().get(&uri).await?;
        wechat_sdk::json_decode(&res)
    }
    // 分阶段发布
    // https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/grayRelease.html

    // 获取分阶段发布详情
    // https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/getGrayReleasePlan.html

    // 设置小程序服务状态
    // https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/setVisitStatus.html

    // 取消分阶段发布
    // https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/revertGrayRelease.html

    // 查询小程序服务状态
    // https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/getVisitStatus.html

    // 查询各版本用户占比
    // https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/getSupportVersion.html

    // 设置最低基础库版本
    // https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/setSupportVersion.html

    // 查询服务商审核额度
    // https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/setCodeAuditQuota.html

    /// 加急代码审核
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/speedupCodeAudit.html
    pub async fn speed_up_audit(&self, auditid: i64) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/speedupaudit?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let data = json!({ "auditid": auditid });

        let res = Client::new().post(&uri, &data).await?;
        wechat_sdk::json_decode(&res)
    }

    // 查询小程序版本信息
    // https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/getVersionInfo.html

    /// 查询最新一次审核单状态
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/getLatestAuditStatus.html
    pub async fn latest_audit_status(&self) -> WechatResult<serde_json::Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/get_latest_auditstatus?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let res = Client::new().get(&uri).await?;
        wechat_sdk::json_decode(&res)
    }

    // 上传提审素材
    // https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/uploadMediaToCodeAudit.html

    // 获取隐私接口检测结果
    // https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/getCodePrivacyInfo.html

    
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
                self.auth_access_token.clone()
            )
        );
        let mut data = json!({
        "path": &path,
        "width": width,
        "auto_color": auto_color,
        "is_hyaline": is_hyaline,
        });
        if auto_color {
            data["line_color"] = serde_json::Value::String(line_color.to_owned());
        }

        Client::new().request_betyes("get", &uri, &data).await
    }
}
