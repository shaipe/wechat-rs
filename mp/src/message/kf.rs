//! copyright © ecdata.cn 2021 - present
//! 客户消息处理

// use std::collections::HashMap;
use wechat_sdk::Client;

pub struct KFService {
    // 公众号的access Token
    access_token: String,
}

impl KFService {
    pub fn new<S: AsRef<str>>(access_token: S) -> Self {
        KFService {
            access_token: access_token.as_ref().to_string(),
        }
    }

    /// 发送客服消息
    #[inline]
    pub async fn send<S: AsRef<str>>(&self, to_user: S, msg_type: S, content: S) {
        let api_url = format!(
            "https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token={}",
            self.access_token
        );
        // let mut params = HashMap::new();
        // params.insert("touser".to_string(), to_user.as_ref().to_string());
        // params.insert("msgtype".to_string(), msg_type.as_ref().to_string());
        // params.insert(msg_type.as_ref().to_string(), content.as_ref().to_string());
        let params = json!({
            "touser": to_user.as_ref().to_string(),
            "msgtype": msg_type.as_ref().to_string(),
            msg_type.as_ref().to_string(): {
                "content": content.as_ref().to_string()
            }
        });
        // let params=format!(r#"{{
        //     "touser":"{}",
        //     "msgtype":"{}",
        //     "{}":{{"content":"{}"}}
        // }}"#,to_user.as_ref().to_string(),
        // msg_type.as_ref().to_string(),
        // msg_type.as_ref().to_string(),
        // content.as_ref().to_string());

        logs!(format!("send kf url:: {} content :: {:?}", api_url, params));

        match Client::new().post(&api_url, &params).await {
            Ok(v) => {
                logs!(format!("success {:?}", v));
            }
            Err(e) => println!("error {:?}", e),
        }
    }
}
