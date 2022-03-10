//! copyright © ecdata.cn 2021 - present
//! 基于actix的client封装的接口请求
//! created by shaipe 20210228

use crate::constant::DEFAULT_USER_AGENT;
use awc::{Client as HttpClient, Connector};
use actix_web::{http::header, http::Method, web::Bytes};

use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use serde::Serialize;

use crate::WechatResult as Result;
use std::time::Duration;
// use tube_value::{ToValue, Value};
use encoding_rs::{Encoding, UTF_8};
use mime::Mime;
use std::borrow::Cow;

/// 请求客户端
#[derive(Clone)]
pub struct Client {
    pub(crate) client: HttpClient,
    charset: String,
}

// pub trait ToResult {
//     fn result(&self) -> Result<ResponseResult>;
// }

// impl tube_error::Result<String, tube_error::Error> for ToResult
// {
//      fn result(&self) -> Result<ResponseResult> {
//         match serde_json::from_str(self) {
//             Ok(rs) => Ok(rs),
//             Err(err) => Err(error! {
//                 code: -1,
//                 msg: format!("error: {}", err)
//             }),
//         }
//     }
// }

/// 将结果转换为字符串
pub(crate) fn text_with_charset(
    headers: &header::HeaderMap,
    default_encoding: &str,
    bs: Bytes,
) -> Result<String> {
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<Mime>().ok());
    let encoding_name = content_type
        .as_ref()
        .and_then(|mime| mime.get_param("charset").map(|charset| charset.as_str()))
        .unwrap_or(default_encoding);
    let encoding = Encoding::for_label(encoding_name.as_bytes()).unwrap_or(UTF_8);

    let (text, _, _) = encoding.decode(&bs);
    if let Cow::Owned(s) = text {
        return Ok(s);
    }
    unsafe {
        // decoding returned Cow::Borrowed, meaning these bytes
        // are already valid utf8
        Ok(String::from_utf8_unchecked(bs.to_vec()))
    }
}

impl Client {
    /// 创建一个新的连接客户端
    pub fn new() -> Self {
        // disable ssl verification
        let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
        builder.set_verify(SslVerifyMode::NONE);
        let _ = builder
            .set_alpn_protos(b"\x02h2\x08http/1.1")
            .map_err(|e| log!("Can not set alpn protocol: {:?}", e));

        let connector = Connector::new()
            .timeout(Duration::from_secs(5))
            .openssl(builder.build())
            ;

        let client = HttpClient::builder()
            .connector(connector)
            .add_default_header((header::USER_AGENT, DEFAULT_USER_AGENT))
            // .header(header::AUTHORIZATION, token)
            // .header(header::REFERER, "http://localhost")
            // .initial_window_size(100)
            // .initial_connection_window_size(100)
            .finish();

        Client {
            client: client,
            charset: "utf-8".to_owned(),
        }
    }

    // pub fn set_reffer(mut self, reffer: String) {
    //     self.client.
    // }

    /// 设置获取数据的编码方式
    pub fn set_charset(mut self, charset: &str) -> Self {
        self.charset = charset.to_owned();
        self
    }

    /// get方式获取站点内容
    pub async fn get(self, url: &str) -> Result<String> {
        match self.client.get(url).send().await {
            Ok(mut res) => {
                // log!("{:?}", res);
                if res.status().is_success() {
                    // match res.json() {
                    //     Ok(res) => {},
                    //     Err(err) => {}
                    // }
                    match res.body().await {
                        Ok(bs) => {
                            let s = text_with_charset(res.headers(), &self.charset, bs);
                            // println!("{:?}", s);
                            return s;
                        }
                        Err(err) => Err(error! {
                            code: -1,
                            msg: format!("error: {}", err)
                        }),
                    }
                } else {
                    Err(error! {
                        code: 500,
                        msg: format!("status={}", res.status())
                    })
                }
            }
            Err(e) => {
                log!("=== request error === {:?}", e);
                Err(error! {
                    code: 500,
                    msg: format!("Send request error: {}", e)
                })
            }
        }
    }

    /// post方式提交数据
    /// url:
    /// param:
    pub async fn post<T: Serialize>(self, url: &str, params: &T) -> Result<String> {
        self.request(Method::POST, url, params).await
    }

    /// 请求put方式
    pub async fn put<T: Serialize>(self, url: &str, params: &T) -> Result<String> {
        self.request(Method::PUT, url, params).await
    }

    /// 请求删除方式
    pub async fn delete<T: Serialize>(self, url: &str, params: &T) -> Result<String> {
        self.request(Method::DELETE, url, params).await
    }

    // /// 解析并转换为tube_value::Value
    // pub fn parse_value(&self, text: &str) -> Result<Value> {

    //     let val: serde_json::Value = match serde_json::from_str(post_str) {
    //         Ok(val) => val,
    //         Err(err) => Err(error! {
    //             code: -1,
    //             msg: format!("error: {}", err)
    //         }),
    //     };

    //     Ok(val.to_value())

    // }

    /// 请求
    pub async fn request_betyes<T: Serialize>(
        self,
        method_str: &str,
        url: &str,
        params: &T,
    ) -> Result<Vec<u8>> {
        // log!("params === {:?}", params);
        let method = match Method::from_bytes(method_str.as_bytes()) {
            Ok(s) => s,
            Err(_e) => Method::POST,
        };
        match self.client.request(method, url).send_json(params).await {
            Ok(mut res) => {
                // log!("response: {:?}", res);
                if res.status().is_success() {
                    match res.body().await {
                        Ok(bs) => Ok(bs.to_vec()),
                        Err(err) => Err(error! {
                            code: -1,
                            msg: format!("error: {}", err)
                        }),
                    }
                } else {
                    Err(error! {
                        code: 500,
                        msg: format!("status={}", res.status())
                    })
                }
            }
            Err(e) => Err(error! {
                code: 500,
                msg: format!("Send request error: {}", e)
            }),
        }
    }

    /// 请求
    pub async fn request<T: Serialize>(
        self,
        method: Method,
        url: &str,
        params: &T,
    ) -> Result<String> {
        // log!("params === {:?}", params);
        match self.client.request(method, url).send_json(params).await {
            Ok(mut res) => {
                // log!("response: {:?}", res);
                if res.status().is_success() {
                    match res.body().await {
                        Ok(bs) => match text_with_charset(res.headers(), &self.charset, bs) {
                            Ok(s) => {
                                // println!(" === {:?}", s);
                                // match serde_json::from_str(&s) {
                                //     Ok(rs) => Ok(rs),
                                //     Err(err) => Err(error! {
                                //         code: -1,
                                //         msg: format!("error: {}", err)
                                //     }),
                                // }
                                Ok(s)
                            }
                            Err(err) => Err(err),
                        },
                        Err(err) => Err(error! {
                            code: -1,
                            msg: format!("error: {}", err)
                        }),
                    }
                } else {
                    Err(error! {
                        code: 500,
                        msg: format!("status={}", res.status())
                    })
                }
            }
            Err(e) => Err(error! {
                code: 500,
                msg: format!("Send request error: {}", e)
            }),
        }
    }

    /// 发送二进制文件
    pub async fn post_betyes(self, url: &str, body: Bytes) -> Result<String> {
        match self.client.post(url).send_body(body).await {
            Ok(mut res) => {
                if res.status().is_success() {
                    match res.body().await {
                        Ok(bs) => {
                            let s = text_with_charset(res.headers(), &self.charset, bs);
                            // println!("{:?}", s);
                            return s;
                        }
                        Err(err) => Err(error! {
                            code: -1,
                            msg: format!("error: {}", err)
                        }),
                    }
                } else {
                    Err(error! {
                        code: 500,
                        msg: format!("status={}", res.status())
                    })
                }
            }
            Err(e) => Err(error! {
                code: 500,
                msg: format!("Send request error: {}", e)
            }),
        }
    }
}
