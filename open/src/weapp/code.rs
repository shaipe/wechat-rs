//! copyright © ecdata.cn 2021 - present
//! 小程序代码管理

use crate::API_DOMAIN;
use serde_json::Value;
use wechat_sdk::{get_url_encode, json_decode, Client, WechatResult};
use super::category::CategoryItem;

pub struct Code {
    auth_access_token: String,
}
impl Code {
    /// 创建对象
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
        ext_json: Value,
        user_version: &str,
        user_desc: &str,
    ) -> WechatResult<Value> {
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

        json_decode(&res)
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
    pub async fn submit_audit(&self, item: CategoryItem) -> WechatResult<Value> {
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
        json_decode(&res)
    }

    /// 查询审核单状态
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/getAuditStatus.html
    pub async fn audit_status(&self, auditid: i64) -> WechatResult<Value> {
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
        json_decode(&res)
    }

    /// 撤回代码审核
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/undoAudit.html
    pub async fn undo_audit(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/undocodeaudit?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let res = Client::new().get(&uri).await?;
        json_decode(&res)
    }

    /// 发布已通过审核的小程序
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/release.html
    pub async fn release(&self) -> WechatResult<Value> {
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
        json_decode(&res)
    }

    /// 小程序版本回退
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/revertCodeRelease.html
    pub async fn revert_code(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/revertcoderelease?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let res = Client::new().get(&uri).await?;
        json_decode(&res)
    }

    /// 分阶段发布
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/grayRelease.html
    /// @param1 - gray_percentage	number	是	灰度的百分比 0~ 100 的整。如果gray_percentage=0，support_experiencer_first与support_debuger_first二选一必填
    /// @param2 - support_debuger_first	boolean	否	true表示支持按体验成员灰度，默认是false
    /// @param3 - support_experiencer_first	boolean	否	true表示支持按项目成员灰度，默认是false
    pub async fn gray_release(
        &self,
        percentage: i32,
        debuuger: bool,
        experiencer: bool,
    ) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/grayrelease?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!({
            "gray_percentage": percentage,
            "support_experiencer_first": experiencer,
            "support_debuger_first": debuuger
        });

        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 获取分阶段发布详情
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/getGrayReleasePlan.html
    pub async fn get_release_plan(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/getgrayreleaseplan?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let res = Client::new().get(&uri).await?;
        json_decode(&res)
    }

    /// 设置小程序服务状态
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/setVisitStatus.html
    /// @param1 - action	string	是	设置可访问状态，发布后默认可访问，close 为不可见，open 为可见
    pub async fn set_visit_status(&self, action: &str) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/change_visitstatus?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let data = json!({ "action": action });
        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 取消分阶段发布
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/revertGrayRelease.html
    pub async fn cancel_gray_release(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/revertgrayrelease?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let res = Client::new().get(&uri).await?;
        json_decode(&res)
    }

    /// 查询小程序服务状态
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/getVisitStatus.html
    pub async fn get_visit_status(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/getvisitstatus?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let data = json!({});
        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 查询各版本用户占比
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/getSupportVersion.html
    pub async fn get_support_version(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/wxopen/getweappsupportversion?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let data = json!({});
        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 设置最低基础库版本
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/setSupportVersion.html
    /// @param1 - version	string	是	为已发布的基础库版本号
    pub async fn set_support_version(&self, version: &str) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/wxopen/setweappsupportversion?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let data = json!({ "version": version });
        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 查询服务商审核额度
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/setCodeAuditQuota.html
    pub async fn get_quota_status(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/queryquota?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let res = Client::new().get(&uri).await?;
        json_decode(&res)
    }
    /// 加急代码审核
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/speedupCodeAudit.html
    pub async fn speed_up_audit(&self, auditid: i64) -> WechatResult<Value> {
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
        json_decode(&res)
    }

    /// 查询小程序版本信息 getVersionInfo
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/getVersionInfo.html
    pub async fn get_version_info(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/getversioninfo?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let data = json!({});
        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 查询最新一次审核单状态
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/getLatestAuditStatus.html
    pub async fn latest_audit_status(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/get_latest_auditstatus?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let res = Client::new().get(&uri).await?;
        json_decode(&res)
    }

    // /// 上传提审素材
    // /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/uploadMediaToCodeAudit.html
    // /// @param1 - media	formdata	是	图片（image）: 2M，支持PNG\JPEG\JPG\GIF格式 视频（video）：10MB，支持MP4格式 完成素材上传后，使用返回的mediaid，可以在提审接口通过post preview_info完成图片和视频上传。 注意：返回的 mediaid 有效期是三天，过期需要重新上传
    // pub async fn upload_media(&self, media: Bytes) -> WechatResult<Value> {
    //     let uri = format!(
    //         "{}{}",
    //         API_DOMAIN,
    //         format!(
    //             "/wxa/uploadmedia?access_token={}",
    //             self.auth_access_token.clone()
    //         )
    //     );
    //     let res = Client::new().post_betyes(&uri, media).await?;
    //     json_decode(&res)
    // }

    /// 获取隐私接口检测结果
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/code-management/getCodePrivacyInfo.html
    pub async fn get_privacy_info(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/security/get_code_privacy_info?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let data = json!({});
        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
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
            data["line_color"] = Value::String(line_color.to_owned());
        }

        Client::new().request_betyes("get", &uri, &data).await
    }
}
