//! copyright © shaipe 2020 - persent
//! 微信对接的网络请求客户端

use crate::errors::WeChatError;
use crate::WeChatResult;
use reqwest::Client as HttpClient;
use std::time::Duration;
// use std::collections::HashMap;
use serde::Serialize;

/// 请求客户端
pub(crate) struct Client {
    pub(crate) client: HttpClient,
}

impl Client {
    pub fn new() -> Self {
        Client {
            client: HttpClient::builder()
                .timeout(Duration::from_secs(300))
                .connect_timeout(Duration::from_secs(300))
                .build()
                .unwrap(),
        }
    }
    /**
     * url
     * params
     */
    pub async fn post<T: Serialize + ?Sized>(&self, url: &str, params: &T) -> WeChatResult<String> {
        match self.client.post(url).json(params).send().await {
            Ok(res) => {
                if res.status() == 200 {
                    match res.text().await {
                        Ok(txt) => {
                            println!("--- {} ----", txt);
                            Ok(txt)
                        }
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
            }),
        }
    }

    /// get方法
    #[allow(dead_code)]
    pub async fn get(&self, url: &str) -> WeChatResult<String> {
        match self.client.get(url).send().await {
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
            }),
        }
    }
}
