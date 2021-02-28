//! copyright © shaipe 2021 - present
//! 微信小程序接口对接
//!

use byteorder::{NativeEndian, ReadBytesExt};
use std::io::Cursor;
use wechat_sdk::{aes128_cbc_decrypt, aes256_cbc_decrypt, Client, WeChatResult};

const API_DOMAIN: &'static str = "https://api.weixin.qq.com";

/// 微信小程序结构体
pub struct WeApp;

impl WeApp {
    pub fn new() -> Self {
        WeApp {}
    }

    /// 注册一个小程序
    /// https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/Mini_Programs/Fast_Registration_Interface_document.html
    pub async fn register(&self) -> WeChatResult<String> {
        Ok("".to_string())
    }

    /// 获取session_key
    pub async fn get_session_key(
        &self,
        appid: &str,
        code: &str,
        component_appid: &str,
        component_access_token: &str,
    ) -> WeChatResult<serde_json::Value> {
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
            Ok(_data) => Ok(_data),
            Err(err) => {
                return Err(err);
            }
        }
    }

    ///获取手机号
    pub fn get_phone_num(
        &self,
        session_key: &str,
        iv: &str,
        encrypte_data: &str,
    ) -> WeChatResult<String> {
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

#[cfg(test)]
mod tests {
    use super::{aes128_cbc_decrypt, aes256_cbc_decrypt, WeApp};

    #[test]
    // async fn test_get_session_key() {
    //     let bll=WeApp::new();
    //     let res=bll.get_session_key("wxcb0d1fe5248ef8c0",
    //     "023g17R91KsWPM1TfJO91flQQ91g17R1",
    //     "wx8c02ccf68ba12555",
    //     "33_M7sY_S52QXyTOOitEkOs-xIYEHVXyn9ZsMTc-Hp9wl4paiLn4kibeR9Ym17zP426FKlgxLVaIlp1IEdBbKs7X6M2lJdURJ_Wb85BrVa3Bf5ypFdC2rN-jzQHfzDfrLr2mXlR_ecq1IH3IW5WZKYhAAANYM"
    // ).await;

    //     println!("decrypted={:?}", res);
    // }
    #[test]
    fn test_get_phone_num() {
        let encrypte_data="6iR9pNmvMgm6L+9NIaumsyqtEzvMgeyCZ+AQLTqjwxwirrgxZJXKvRyDCD3cU97hcVN1bzZrTYryxul40TrBqneGDjyiV4wVO+mF+Vx9xC9Sxa9FmMB+Eqgt47Wy5Qr7Jy/YvTFffxxsGpTT01zreQUov7N3tGFckWWAr+FqO3UbtRuGrTdEjX7TrtTl6jTUoJ57e9rsLL6SEfdQ6omliw==";
        let session_key = "QCTvQOXnZugJc4XezoCKUQ==";
        let iv = "M/Omw6FuQ0YU8Z2KAibRLA==";
        let encrypte_data = base64::decode(encrypte_data).unwrap();
        let iv = base64::decode(iv).unwrap();
        let session_key = base64::decode(session_key).unwrap();
        println!("session_key={:?}", session_key);
        // aes descrypt
        let res = aes128_cbc_decrypt(&encrypte_data, &session_key, &iv[..16]);
        println!("res={:?}", res);
        let text = match res {
            Ok(v) => match String::from_utf8(v) {
                Ok(s) => {
                    println!("{:?}", s);
                    s
                }
                Err(_) => "".to_owned(),
            },
            Err(_) => "".to_owned(),
        };
        println!("{:?}", text);
    }
}
