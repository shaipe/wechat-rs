//! copyright
//! 微信小程序接口对接

use crate::WeChatResult;

pub struct WeApp{}

impl WeApp
{
    pub fn new() -> Self {
        WeApp{}
    }

    /// 注册一个小程序
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/Mini_Programs/Fast_Registration_Interface_document.html
    pub async fn register(&self) -> WeChatResult<String> {
        Ok("".to_string())
    }
}