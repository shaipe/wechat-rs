//! copyright © ecdata.cn 2021 - present
//! 小程序域名设置

use serde_json::Value;
use wechat_sdk::{json_decode, Client, WechatResult};
use crate::API_DOMAIN;

pub struct Basic {
    auth_access_token: String,
}

impl Basic {
    /// 创建对象
    pub fn new(_auth_access_token: &str) -> Self {
        Basic {
            auth_access_token: _auth_access_token.to_string(),
        }
    }

    /// 获取基本信息
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Mini_Program_Basic_Info/Mini_Program_Information_Settings.html
    pub async fn get_basic_info(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/account/getaccountbasicinfo?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let res = Client::new().get(&uri).await?;
        json_decode(&res)
    }

    /// 查询公众号/小程序是否绑定open帐号
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Mini_Program_Basic_Info/getbindopeninfo.html
    pub async fn get_have_bind_open(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/open/have?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let res = Client::new().get(&uri).await?;
        json_decode(&res)
    }

    /// 小程序名称检测
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/basic-info-management/checkNickName.html
    /// @param1 - nick_name	string	是	名称（昵称）
    pub async fn check_nick_name(&self, nick_name: &str) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/wxverify/checkwxverifynickname?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
            "nick_name": nick_name
        });

        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 设置小程序名称
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/basic-info-management/setNickName.html
    /// nick_name	string	是	昵称，不支持包含“小程序”关键字的昵称
    pub async fn set_nick_name(&self, nick_name: &str) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/setnickname?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
            "nick_name": nick_name
        });

        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 查询小程序名称审核状态
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/basic-info-management/getNickNameStatus.html
    /// @param1 - audit_id	number	是	审核单 id，由设置名称接口返回
    pub async fn query_nick_name(&self, audit_id: &str) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/api_wxa_querynickname?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
            "audit_id": audit_id
        });

        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 设置小程序介绍
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/basic-info-management/setSignature.html
    /// @param1 - signature	string	是	功能介绍（简介）
    pub async fn set_intro(&self, content: &str) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/account/modifysignature?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
            "signature": content
        });

        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 获取搜索状态
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/basic-info-management/getSearchStatus.html
    ///  
    pub async fn get_search_status(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/getwxasearchstatus?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let res = Client::new().get(&uri).await?;
        Ok(json_decode(&res)?["status"].clone())
    }

    /// 设置可搜索状态
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/basic-info-management/setSearchStatus.html
    /// @param1 - status	number	是	1 表示不可搜索，0 表示可搜索
    pub async fn set_search_status(&self, status: i8) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/changewxasearchstatus?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
            "status": status
        });

        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 修改头像
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/basic-info-management/setHeadImage.html
    /// @param1 - head_img_media_id	string	是	头像素材 media_id
    pub async fn set_head_image(&self, media_id: i8) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/cgi-bin/account/modifyheadimage?access_token={}",
                self.auth_access_token.clone()
            )
        );

        let data = json!(
        {
            "head_img_media_id": media_id,
            "x1": "0",
            "y1": "0",
            "x2": "1",
            "y2": "1"
        });

        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }


    

    
    

    // // 名称审核结果事件推送接收
    // // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Mini_Program_Basic_Info/wxa_nickname_audit.html

    // // 查询改名审核状态
    // // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Mini_Program_Basic_Info/api_wxa_querynickname.html

    // // 修改头像
    // // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Mini_Program_Basic_Info/modifyheadimage.html

    // // 修改功能介绍
    // // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Mini_Program_Basic_Info/modifysignature.html
}
