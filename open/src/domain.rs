//! copyright © ecdata.cn 2022 - present
//! 第三方平台域名管理
//!

/// 域名管理
pub struct Domain {
    comp_access_token: String,
}

impl Domain {
    /// 创建域名管理对象
    pub fn new(access_token: &str) -> Self {
        Domain {
            comp_access_token: access_token.to_owned(),
        }
    }

    // 设置第三方平台服务器域名
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/ThirdParty/domain/modify_server_domain.html

    // 获取第三方业务域名的校验文件
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/ThirdParty/domain/get_domain_confirmfile.html

    // 设置第三方平台业务域名
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/ThirdParty/domain/modify_jump_domain.html
}
