//! copyright © ecdata.cn 2022 - present
//! 代商家注册小程序
//! 

/// 代商家注册小程序
pub struct Register {
    comp_access_token: Sting
}

impl Register {
    pub fn new(access_token: &str) -> Self {
        Register {
            comp_access_token: access_token.to_owned()
        }
    }

    // 快速注册企业小程序
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Register_Mini_Programs/Fast_Registration_Interface_document.html

    // 复用公众号主体快速注册小程序
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Register_Mini_Programs/fast_registration_of_mini_program.html

    // 快速创建个人小程序
    // https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/api/Register_Mini_Programs/fastregisterpersonalweapp.html
    
}