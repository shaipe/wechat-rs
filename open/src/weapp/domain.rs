//! copyright © ecdata.cn 2022 - present
//!
//!
//!

use serde_json::Value;
use wechat_sdk::{json_decode, Client, WechatResult};

use crate::API_DOMAIN;

pub struct Domain {
    auth_access_token: String,
}

impl Domain {
    /// 创建对象
    pub fn new(auth_access_token: &str) -> Self {
        Domain {
            auth_access_token: auth_access_token.to_owned(),
        }
    }

    /// 配置小程序服务器域名
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/domain-management/modifyServerDomain.html
    pub async fn set_server_domain(
        &self,
        req_domain: Vec<String>,
        ws_domain: Vec<String>,
        upload_domain: Vec<String>,
        down_domain: Vec<String>,
    ) -> WechatResult<Value> {
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
        json_decode(&res)
    }

    /// 配置小程序业务域名
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/domain-management/modifyJumpDomain.html
    /// @param1 - webviewdomain	array<string>	否	小程序业务域名，当 action 参数是 get 时不需要此字段
    pub async fn set_webview_domain(&self, req_domain: Vec<String>) -> WechatResult<Value> {
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
            "webviewdomain": req_domain
        });

        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 快速配置小程序服务器域名
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/domain-management/modifyServerDomainDirectly.html
    /// @param1 - action	string	是	操作类型 add: 添加, delete: 删除, set: 覆盖,ge:获取
    /// @param2 - requestdomain	array<string>	是	request 合法域名；当 action 是 get 时不需要此字
    /// @param3 - wsrequestdomain	array<string>	是	socket 合法域名；当 action 是 get 时不需要此字段
    /// @param4 - uploaddomain	array<string>	是	uploadFile 合法域名；当 action 是 get 时不需要此字段
    /// @param5 - downloaddomain	array<string>	是	downloadFile 合法域名；当 action 是 get 时不需要此字段
    /// @param6 - udpdomain	array<string>	是	udp 合法域名；当 action 是 get 时不需要此字段
    /// @param7 - tcpdomain	array<string>	是	tcp 合法域名；当 action 是 get 时不需要此字段
    pub async fn set_domains(
        &self,
        action: &str,
        req_domain: Vec<String>,
        ws_domain: Vec<String>,
        upload_domain: Vec<String>,
        down_domain: Vec<String>,
        udp_domain: Vec<String>,
        tcp_domain: Vec<String>,
    ) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/modify_domain_directly?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let data = json!({
            "action": action,
            "requestdomain": req_domain,
            "wsrequestdomain": ws_domain,
            "uploaddomain": upload_domain,
            "downloaddomain": down_domain,
            "udpdomain": udp_domain,
            "tcpdomain": tcp_domain
        });

        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 获取业务域名校验文件 
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/domain-management/getJumpDomainConfirmFile.html
    pub async fn get_confirm_file(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/get_webviewdomain_confirmfile?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let res = Client::new().get(&uri).await?;
        json_decode(&res)
    }
    
    /// 快速配置小程序业务域名
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/domain-management/modifyJumpDomainDirectly.html
    pub async fn set_speed_webview_domain(&self, req_domain: Vec<String>) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/setwebviewdomain_directly?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let data = json!({
            "action":"add",
            "webviewdomain": req_domain
        });

        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

    /// 获取发布后生效服务器域名列表
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/domain-management/getEffectiveServerDomain.html
    pub async fn get_effective_domain(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/get_effective_domain?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let res = Client::new().post(&uri, &json!({})).await?;
        json_decode(&res)
    }

    /// 获取发布后生效业务域名列表
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/domain-management/getEffectiveJumpDomain.html
    pub async fn get_effective_webview_domain(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/get_effective_webviewdomain?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let res = Client::new().post(&uri, &json!({})).await?;
        json_decode(&res)
    }

    /// 获取 DNS 预解析域名
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/domain-management/getPrefetchDomain.html
    pub async fn get_prefetch_dns(&self) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/get_prefetchdnsdomain?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let res = Client::new().get(&uri).await?;
        json_decode(&res)
    }

    /// 设置 DNS 预解析域名
    /// https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/domain-management/setPrefetchDomain.html
    /// @param1 - refetch_dns_domain	Array.<object>	是	预解析域名
    pub async fn set_prefetch_dns(&self, domains: Vec<String>) -> WechatResult<Value> {
        let uri = format!(
            "{}{}",
            API_DOMAIN,
            format!(
                "/wxa/set_prefetchdnsdomain?access_token={}",
                self.auth_access_token.clone()
            )
        );
        let data = json!({
            "prefetch_dns_domain": domains
        });

        let res = Client::new().post(&uri, &data).await?;
        json_decode(&res)
    }

}
