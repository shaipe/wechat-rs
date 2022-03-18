//! copyright © ecdata.cn 2021 - present
//! 小程序授权接口对接
//! DOC https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/login/auth.code2Session.html
//!

// use byteorder::{NativeEndian, ReadBytesExt};
// use std::io::Cursor;
use wechat_sdk::{AesCrypt, Client, WechatResult};

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
        api= crate::API_DOMAIN,
        appid=appid,
        code=code,
        secret=secret
    );
        let api = Client::new();
        let res = api.get(&url).await?;
        match wechat_sdk::json_decode(&res) {
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


    /// 解析小程的手机号数据
    pub fn parse_phone_number(
        encrypt_text: &str,
        session_key: &str,
        iv: &str,
    ) -> WechatResult<serde_json::Value> {
        let keys = base64::decode(session_key).unwrap();
        let ivs = base64::decode(iv).unwrap();
        let aes=AesCrypt::new(keys,ivs);
        let content=aes.decrypt(encrypt_text.to_owned());
        let v =content.as_bytes(); 

            // log!("v {:?} str {:?}", &v, std::str::from_utf8(&v));
        // 需要后前移7位,不解出来的对象不是正确的json
        // 对称解密使用的算法为 AES-128-CBC，数据采用PKCS#7填充
        let xv = &v[0..v.len() - 14];
        // log!("v {:?}", xv);
        match std::str::from_utf8(xv) {
            Ok(s) => {
                let val: serde_json::Value = match serde_json::from_str(s) {
                    Ok(v) => v,
                    Err(err) => return Err(error!("parse json error: {:?}", err)),
                };
                Ok(val)
            }
            Err(e) => return Err(error!("parse string failed: {:?}", e)),
        }
    }
}
