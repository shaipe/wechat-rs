//! copyright © ecdata.cn 2021 - present
//! 微信开放平台的第三方平代小程序处理的业务
//!


use byteorder::{NativeEndian, ReadBytesExt};
use std::io::Cursor;
use wechat_sdk::{aes128_cbc_decrypt, aes256_cbc_decrypt, Client, WechatResult};

const API_DOMAIN: &'static str = "https://api.weixin.qq.com";

pub struct WxApp;

impl WxApp {
    /// 获取session_key
    /// GET https://api.weixin.qq.com/sns/jscode2session?appid=APPID&secret=SECRET&js_code=JSCODE&grant_type=authorization_code
    /// 登录凭证校验。通过 wx.login 接口获得临时登录凭证 code 后传到开发者服务器调用此接口完成登录流程。
    /// DOC: https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/login/auth.code2Session.html
    /// @param1: 小程序appid
    /// @param2: wx.login()获取的code
    /// @param3: 
    pub async fn get_session_key(
        &self,
        appid: &str,
        code: &str,
        component_appid: &str,
        component_access_token: &str,
    ) -> WechatResult<serde_json::Value> {
        let url = format!("{api}/sns/component/jscode2session?appid={appid}&js_code={code}&grant_type=authorization_code&component_appid={component_appid}&component_access_token={component_access_token}",
        api=API_DOMAIN,
        appid=appid,
        code=code,
        component_appid=component_appid,
        component_access_token=component_access_token
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


    /// 获取手机号
    /// @param1: 上一步获取的sessionKey
    /// @param2: 解密向量
    /// @param3: 加密数据，将对此数据进行解密使用
    pub fn get_phone_num(
        &self,
        session_key: &str,
        iv: &str,
        encrypte_data: &str,
    ) -> WechatResult<String> {
        let encrypte_data = base64::decode(encrypte_data).unwrap();
        let iv = base64::decode(iv).unwrap();
        let session_key = base64::decode(session_key).unwrap();
        println!("session_key={:?}", session_key);
        // aes descrypt
        let res = aes128_cbc_decrypt(&encrypte_data, &session_key, &iv);
        println!("res={:?}", res);
        let text = match res {
            Ok(v) => {
                // let mut rdr = Cursor::new(v[16..20].to_vec());
                // let content_length = u32::from_be(rdr.read_u32::<NativeEndian>().unwrap()) as usize;
                // println!("content_length={}",content_length);
                // let content = &v[20..];

                // let content_string = String::from_utf8(content.to_vec()).unwrap();
                //content_string
                match String::from_utf8(v) {
                    Ok(s) => s,
                    Err(_) => "".to_owned(),
                }
            }
            Err(_) => "".to_owned(),
        };
        println!("{:?}", text);

        Ok(text)
    }
}