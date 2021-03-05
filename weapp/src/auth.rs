//! copyright © ecdata.cn 2021 - present
//! 小程序授权接口对接
//! DOC https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/login/auth.code2Session.html
//!

// use byteorder::{NativeEndian, ReadBytesExt};
// use std::io::Cursor;
use wechat_sdk::{Client, WechatResult};

const API_DOMAIN: &'static str = "https://api.weixin.qq.com";

pub struct Auth;

impl Auth {
    /// 获取session_key
    /// GET https://api.weixin.qq.com/sns/jscode2session?appid=APPID&secret=SECRET&js_code=JSCODE&grant_type=authorization_code
    /// 登录凭证校验。通过 wx.login 接口获得临时登录凭证 code 后传到开发者服务器调用此接口完成登录流程。
    /// DOC: https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/login/auth.code2Session.html
    /// @param1: appid	string		是	小程序 appId
    /// @param2: secret	string		是	小程序 appSecret
    /// @param3: js_code	string		是	登录时获取的 code
    /// @param4: 无需传入,grant_type	string		是	授权类型，此处只需填写 authorization_code
    pub async fn get_session_key(
        appid: &str,
        secret: &str,
        code: &str,
    ) -> WechatResult<serde_json::Value> {
        let url = format!("{api}/sns/jscode2session?appid={appid}&secret={secret}&js_code={code}&grant_type=authorization_code",
        api=API_DOMAIN,
        appid=appid,
        code=code,
        secret=secret
    );
        let api = Client::new();
        let res = api.get(&url).await?;
        match api.json_decode(&res) {
            Ok(data) => Ok(data),
            Err(err) => {
                return Err(err);
            }
        }
    }

    /// 用户支付完成后，获取该用户的 UnionId，无需用户授权。本接口支持第三方平台代理查询。
    /// GET https://api.weixin.qq.com/wxa/getpaidunionid?access_token=ACCESS_TOKEN&openid=OPENID
    /// DOC https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/user-info/auth.getPaidUnionId.html
    pub fn get_paid_union_id() -> WechatResult<String> {
        Ok("".to_string())
    }

    /// 获取小程序全局唯一后台接口调用凭据（access_token）。调用绝大多数后台接口时都需使用 access_token，开发者需要进行妥善保存。
    /// GET https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid=APPID&secret=APPSECRET
    /// DOC https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/access-token/auth.getAccessToken.html
    pub fn get_access_token() -> WechatResult<String> {
        Ok(String::from(""))
    }
}
