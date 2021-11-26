//! copyright © ecdata.cn 2021 - present
//! 小程序码
//! created by shaipe 20210228
//! https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/qr-code/wxacode.createQRCode.html
//! createQRCode
//! get
//! getUnlimited
//! urlscheme.generate
//! 获取小程序scheme码，适用于短信、邮件、外部网页等拉起小程序的业务场景。通过该接口，可以选择生成到期失效和永久有效的小程序码，目前仅针对国内非个人主体的小程序开放，详见获取URL scheme码。
//!
//!

use wechat_sdk::{constant::API_DOMAIN, AccessToken, Client, WechatResult};

#[derive(Debug, Clone)]
pub struct QRCode {
    /// 应用id
    pub app_id: String,
    /// 密钥
    pub secret: String,
}

impl QRCode {
    /// 创建一个二维码
    pub fn new(app_id: &str, secret: &str) -> QRCode {
        QRCode {
            app_id: app_id.to_owned(),
            secret: secret.to_owned(),
        }
    }

    /// 获取接口access token
    async fn get_access_token(&self) -> String {
        match AccessToken::new("weapp", &self.app_id, &self.secret)
            .get()
            .await
        {
            Some(s) => s.access_token,
            None => "".to_owned(),
        }
    }
    /// Create a new QRCode
    /// path	string		是	扫码进入的小程序页面路径，最大长度 128 字节，不能为空；对于小游戏，可以只传入 query 部分，来实现传参效果，如：传入 "?foo=bar"，即可在 wx.getLaunchOptionsSync 接口中的 query 参数获取到 {foo:"bar"}。
    /// width	number	430	否	二维码的宽度，单位 px。最小 280px，最大 1280px
    /// api url: https://api.weixin.qq.com/cgi-bin/wxaapp/createwxaqrcode?access_token=ACCESS_TOKEN
    pub async fn create_qrcode(&self, path: &str, width: usize) -> WechatResult<Vec<u8>> {
        let access_token = self.get_access_token().await;
        if access_token == "" {
            return Err(error!("Access token not found"));
        }

        let url = format!(
            "{}/cgi-bin/wxaapp/createwxaqrcode?access_token={}",
            API_DOMAIN, access_token
        );
        // 接口参数
        let val = json!({
            "path": path,
            "width": width
        });
        // log!("=== url === {}", url);
        match Client::new().request_betyes("POST", &url, &val).await {
            Ok(res) => {
                // let base64_str= tube_img::vec_to_base64(res);
                // log!("{:?}", base64_str);
                Ok(res)
            }
            Err(err) => Err(err),
        }
    }

    // 转为bas64
    pub fn base64_image_bytes(_bs: Vec<u8>) -> String {
        // tube_img::vec_to_base64(bs)
        "".to_string()
    }

    /// 获取小程序码，适用于需要的码数量极多的业务场景
    /// https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/qr-code/wxacode.getUnlimited.html
    /// auto_color	boolean	false	否	自动配置线条颜色，如果颜色依然是黑色，则说明不建议配置主色调，默认 false
    /// line_color	Object	{"r":0,"g":0,"b":0}	否	auto_color 为 false 时生效，使用 rgb 设置颜色 例如 {"r":"xxx","g":"xxx","b":"xxx"} 十进制表示
    /// is_hyaline	boolean	false	否	是否需要透明底色，为 true 时，生成透明底色的小程序
    pub async fn get_unlimited_qrcode(
        &self,
        sence: &str,
        path: &str,
        width: usize,
    ) -> WechatResult<Vec<u8>> {
        let access_token = self.get_access_token().await;
        if access_token == "" {
            return Err(error!("Access token not found"));
        }

        let url = format!(
            "{}/wxa/getwxacodeunlimit?access_token={}",
            API_DOMAIN, access_token
        );
        // 接口参数
        let val = json!({
            "sence": sence,
            "path": path,
            "width": width
        });
        // log!("=== url === {}", url);
        match Client::new().request_betyes("POST", &url, &val).await {
            Ok(res) => Ok(res),
            Err(err) => Err(err),
        }
    }

    /// 获取小程序码，适用于需要的码数量极多的业务场景
    /// https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/qr-code/wxacode.get.html
    /// auto_color	boolean	false	否	自动配置线条颜色，如果颜色依然是黑色，则说明不建议配置主色调，默认 false
    /// line_color	Object	{"r":0,"g":0,"b":0}	否	auto_color 为 false 时生效，使用 rgb 设置颜色 例如 {"r":"xxx","g":"xxx","b":"xxx"} 十进制表示
    /// is_hyaline	boolean	false	否	是否需要透明底色，为 true 时，生成透明底色的小程序
    pub async fn get(&self, path: &str, width: usize) -> WechatResult<Vec<u8>> {
        // 获取access token
        let access_token = self.get_access_token().await;
        if access_token == "" {
            return Err(error!("Access token not found"));
        }

        // url地址拼装
        let url = format!(
            "{}/wxa/getwxacode?access_token={}",
            API_DOMAIN, access_token
        );
        // 接口参数
        let val = json!({
            "path": path,
            "width": width
        });
        // log!("=== url === {}", url);
        match Client::new().request_betyes("POST", &url, &val).await {
            Ok(res) => Ok(res),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use actix_web::rt;
    #[test]
    fn test() {
        // 订单查询
        // let x = rt::System::new("").block_on(Order::query("sdsdsd"));
        // let qr = QRCode::new("appid", "secret");
        // let x = rt::System::new("").block_on(
        //     // qr.create_qrcode("pages/index/index?invte=0900", 200),
        // );
        // log!("result === {:?}", x);
    }
}
