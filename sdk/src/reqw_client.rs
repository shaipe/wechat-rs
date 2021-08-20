//! copyright © shaipe 2020 - persent
//! 微信对接的网络请求客户端

use crate::WechatResult;
use bytes::Bytes;
use reqwest::header;
use reqwest::Client as HttpClient;
use serde::Serialize;
use std::time::Duration;

const DEFAULT_USER_AGENT: &'static str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3534.4 Safari/537.36";

/// 请求客户端
pub struct Client {
    pub(crate) client: HttpClient,
}

impl Client {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(DEFAULT_USER_AGENT),
        );

        Client {
            client: HttpClient::builder()
                .timeout(Duration::from_secs(300))
                .connect_timeout(Duration::from_secs(300))
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }
    /// post方式提交数据
    /// url:
    /// param:
    pub async fn post<T: Serialize + ?Sized>(&self, url: &str, params: &T) -> WechatResult<String> {
        match self.client.post(url).json(params).send().await {
            Ok(res) => {
                if res.status() == 200 {
                    match res.text().await {
                        Ok(txt) => {
                            // println!("--- {} ----", txt);
                            Ok(txt)
                        }
                        Err(e) => Err(error! {
                            code: -1,
                            msg: format!("Send request error: {}", e),
                        }),
                    }
                } else {
                    Err(error! {
                        code: 500,
                        msg: format!("status={}", res.status()),
                    })
                }
            }
            Err(e) => Err(error! {
                code: 500,
                msg: format!("Send request error: {}", e),
            }),
        }
    }

    /// 发送二进制文件
    pub async fn post_betyes(&self, url: &str, body: Bytes) -> WechatResult<String> {
        match self.client.post(url).body(body).send().await {
            Ok(res) => {
                if res.status() == 200 {
                    match res.text().await {
                        Ok(txt) => Ok(txt),
                        Err(e) => Err(error! {
                            code: -1,
                            msg: format!("Send request error: {}", e),
                        }),
                    }
                } else {
                    Err(error! {
                        code: 500,
                        msg: format!("status={}", res.status()),
                    })
                }
            }
            Err(e) => Err(error! {
                code: 500,
                msg: format!("Send request error: {}", e),
            }),
        }
    }

    /// get方法
    #[allow(dead_code)]
    pub async fn get(&self, url: &str) -> WechatResult<String> {
        match self.client.get(url).send().await {
            Ok(res) => {
                if res.status() == 200 {
                    match res.text().await {
                        Ok(txt) => Ok(txt),
                        Err(e) => Err(error! {
                            code: -1,
                            msg: format!("Send request error: {}", e),
                        }),
                    }
                } else {
                    Err(error! {
                        code: 500,
                        msg: format!("status={}", res.status()),
                    })
                }
            }
            Err(e) => Err(error! {
                code: 500,
                msg: format!("Send request error: {}", e),
            }),
        }
    }

    
}
