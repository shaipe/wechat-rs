//! copyright © ecdata.cn 2021 - present
//! 接口凭证管理，直接使用文件进行管理
//! created by shaipe 20211012

use crate::{constant::API_DOMAIN, Client, WechatResult as Result};
use std::collections::HashMap;
use std::sync::Mutex;

// 缓存key
const WECHAT_CONFIG_KEY: &'static str = "wechat_config_cache";

// 默认加载静态全局
lazy_static! {
    pub static ref WECHAT_CONFIG_CACHES: Mutex<HashMap<String, Token>> = Mutex::new(HashMap::new());
}

/// Access Token对象
#[derive(Debug, Clone, Default)]
pub struct AccessToken {
    // 应用类型
    pub app_type: String,
    // 应用id
    pub app_id: String,
    // 密钥
    pub secret: String,
}

/// Token信息
#[derive(Debug, Clone, Default)]
pub struct Token {
    // 访问token
    pub access_token: String,
    // access_token获取时间
    pub create_time: i64,
    // 有效期
    pub expires: i64,
}

impl AccessToken {
    /// 创建一个短信配置实例
    pub fn new(app_type: &str, app_id: &str, secret: &str) -> AccessToken {
        AccessToken {
            app_type: app_type.to_owned(),
            app_id: app_id.to_owned(),
            secret: secret.to_owned(),
        }
    }

    /// 获取微信授权的accessToken
    pub async fn get_access_token(&self, grant_type: &str) -> Result<Token> {
        // 组装请求地址
        let url = format!(
            "{domain}/cgi-bin/token?grant_type={grant_type}&appid={app_id}&secret={secret}",
            domain = API_DOMAIN,
            grant_type = if grant_type == "" {
                "client_credential"
            } else {
                grant_type
            },
            app_id = self.app_id,
            secret = self.secret
        );

        // 调用远程接口
        match Client::new().get(&url).await {
            Ok(res) => {
                match crate::json_decode(&res) {
                    Ok(data) => {
                        let token = match data["access_token"].as_str(){
                            Some(s) => s.to_owned(),
                            None => return Err(error!("access token error"))
                        };

                        // 将Token返出去
                        return Ok(Token {
                            access_token: token,
                            create_time: crate::current_timestamp(),
                            expires: 7200,
                        });
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            Err(err) => log!("error{:?}", err),
        }

        Err(error!("access token is invalid"))
    }

    /// 把字符串对象写入缓存中,并指定有有效期单位秒
    pub fn set(&self, val: Token) {
        let key = WECHAT_CONFIG_KEY;
        // log!("setting config");
        WECHAT_CONFIG_CACHES
            .lock()
            .unwrap()
            .insert(key.to_owned(), val);
        // log!("setted config");
    }

    /// 获取cache中的缓存数据
    pub async fn get(&self) -> Option<Token> {
        let key = WECHAT_CONFIG_KEY;
        let cache = WECHAT_CONFIG_CACHES.lock().unwrap();

        if let Some(cnf) = cache.get(key) {
            return Some(cnf.clone());
        } else {
            match self.get_access_token("client_credential").await {
                Ok(access) => {
                    return Some(access.clone());
                }
                Err(err) => {
                    log!("get access token error{:?}", err);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut conf = AccessToken::default();
        // conf.username = "dlxumin1".to_owned();
        // conf.password = "dlxumin1123".to_owned();
        // conf.sign = "宏推".to_owned();
        // let _ = conf.save();

        println!("test");
    }
}
