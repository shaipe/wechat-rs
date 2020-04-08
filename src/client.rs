//! copyright © shaipe 2020 - persent

use reqwest::Client as HttpClient;
use std::time::Duration;
use crate::errors::WeChatError;
use crate::WeChatResult;

use std::collections::HashMap;
use rustc_serialize::json::{Json};

/// 请求客户端
pub(crate) struct Client {
    pub(crate) client: HttpClient,
}

impl Client {
    pub fn new() -> Self {
        Client {
            client: HttpClient::builder()
                .timeout(Duration::from_secs(5))
                .connect_timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
        }
    }
    /**
     * url
     * params
     */
    pub async fn post(&self, url: &str, params: &HashMap<String, String>) -> WeChatResult<String> {
  
       match self.client
            .post(url)
            .json(params)
            .send().await
        {
            Ok(res) => {
                if res.status() == 200 {
                    match res.text().await {
                        Ok(txt) => Ok(txt),
                        Err(e) => Err(WeChatError::ClientError {
                            errcode: -1,
                            errmsg: format!("Send request error: {}", e),
                        }),
                    }
                } else {
                    Err(WeChatError::ClientError {
                        errcode: 500,
                        errmsg: format!("status={}", res.status()),
                    })
                }
            }
            Err(e) => Err(WeChatError::ClientError {
                errcode: 500,
                errmsg: format!("Send request error: {}", e),
            })
        }
      
    }

    /// get方法
    #[allow(dead_code)]
    pub async fn get(&self,url: &str) -> WeChatResult<String> {
       
        match self.client
            .get(url)
            .send().await
            {
                Ok(res) => {
                    if res.status() == 200 {
                        match res.text().await{
                            Ok(txt) => Ok(txt),
                            Err(e) =>Err(WeChatError::ClientError { errcode: -1, errmsg: format!("Send request error: {}", e) })
                        }
                    } else {
                        Err(WeChatError::ClientError { errcode: 500, errmsg: format!("status={}",res.status()) })
                    }
                },
                Err(e)=>Err(WeChatError::ClientError { errcode: 500, errmsg: format!("Send request error: {}", e) })
            }
    }

    #[inline]
    pub fn json_decode(&self,data:&str) -> WeChatResult<Json> {
        let obj = match Json::from_str(data) {
            Ok(decoded) => { decoded },
            Err(ref e) => {
                return Err(WeChatError::ClientError { errcode: -3, errmsg: format!("Json decode error: {}", e) });
            }
        };
        match obj.find("errcode") {
            Some(code) => {
                let errcode = code.as_i64().unwrap();
                if errcode != 0 {
                    let errmsg = match obj.find("errmsg") {
                        Some(msg) => {
                            msg.as_string().unwrap()
                        },
                        None => { "" }
                    };
                    return Err(WeChatError::ClientError { errcode: errcode as i32, errmsg: errmsg.to_owned() });
                }
            },
            None => {},
        };
        Ok(obj)
   
    }
}
