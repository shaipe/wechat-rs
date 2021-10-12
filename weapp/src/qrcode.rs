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

use wechat_sdk::{aes128_cbc_decrypt, Client, WechatResult};

pub struct QRCode;

impl QRCode {
    /// Create a new QRCode
    /// path	string		是	扫码进入的小程序页面路径，最大长度 128 字节，不能为空；对于小游戏，可以只传入 query 部分，来实现传参效果，如：传入 "?foo=bar"，即可在 wx.getLaunchOptionsSync 接口中的 query 参数获取到 {foo:"bar"}。
    /// width	number	430	否	二维码的宽度，单位 px。最小 280px，最大 1280px
    /// api url: https://api.weixin.qq.com/cgi-bin/wxaapp/createwxaqrcode?access_token=ACCESS_TOKEN
    pub async fn create_qrcode(path: &str, width: usize) -> WechatResult<serde_json::Value> {
        let url = format!(
            "{}/cgi-bin/wxaapp/createwxaqrcode?access_token=ACCESS_TOKEN",
            crate::API_DOMAIN
        );
        // 接口参数
        let val = json!({
            "path": path,
            "width": width
        });
        match Client::new().post(&url, &val).await {
            Ok(res) => match wechat_sdk::json_decode(&res) {
                Ok(data) => Ok(data),
                Err(err) => {
                    return Err(err);
                }
            },
            Err(err) => Err(err),
        }
    }
}
