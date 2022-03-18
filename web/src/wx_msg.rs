//! copyright
//!
//! 微信消息处理
//!

use super::utils;
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use std::collections::HashMap;
use url::Url;
use wechat::{
    mp::message::{KFService, Message, ReplyRender, TextReply},
    open::{get_tripartite_config, Component, TripartiteConfig},
};
use wechat_sdk::{current_timestamp, WeChatCrypto};
use redis::{
    get_redis_conf,
    RedisConfig
};

/// 消息回复处理
pub async fn message_reply(msg: &Message) -> Result<HttpResponse> {
    match msg {
        Message::TextMessage(ref m) => {
            println!("$$$$$$$$$$$$$$ text message $$$$$$$$$$$$$$\n{:?}", m);
            // let tr = TextReply::new(
            //     &m.to_user,
            //     &m.from_user,
            //     &format!("{}_callback", &m.content),
            // );
            // let txt = tr.render();
            // let timestamp = current_timestamp();
            // let encrypt_text = c.encrypt_message(&txt, timestamp, &nonce);
            // println!("---- send text reply xml :{}", txt);
            // return Ok(HttpResponse::build(StatusCode::OK)
            //     .content_type("text/xml; charset=utf-8")
            //     .body(encrypt_text.unwrap()));
        }
        Message::EventMessage(ref m) => println!("{:?}", m),
        Message::UnknownMessage(ref m) => println!("{:?}", m),
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("success"))
}

// async fn forward(
//     req: HttpRequest,
//     pr: web::Data<Proxy>,
//     body: web::Bytes,
//     // url: web::Data<Url>,
//     client: web::Data<Client>,
// ) -> Result<HttpResponse, Error> {
//     // println!("{:?}", pr);

//     // 代理请求目标主机集群
//     let targets = &pr.target;
//     // 获取第一个目标主机
//     let target_url = &targets[0];
//     // 创建一个可变的url地址
//     let mut new_url = Url::parse(&target_url.url).unwrap();

//     let url_path = req.uri().path();
//     let start_index = pr.path.len();
//     // 设置请求的路径,并去掉代理目录前缀
//     new_url.set_path(&url_path[start_index..]);
//     new_url.set_query(req.uri().query());
//     // let mut new_url = url.get_ref().clone();
//     // new_url.set_path(req.uri().path());
//     // new_url.set_query(req.uri().query());

//     println!("{:?}", new_url);

//     // TODO: This forwarded implementation is incomplete as it only handles the inofficial
//     // X-Forwarded-For header but not the official Forwarded one.
//     let forwarded_req = client
//         .request_from(new_url.as_str(), req.head())
//         .no_decompress();
//     let forwarded_req = if let Some(addr) = req.head().peer_addr {
//         forwarded_req.header("x-forwarded-for", format!("{}", addr.ip()))
//     } else {
//         forwarded_req
//     };

//     let mut res = forwarded_req.send_body(body).await.map_err(Error::from)?;

//     let mut client_resp = HttpResponse::build(res.status());
//     // Remove `Connection` as per
//     // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection#Directives
//     for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
//         client_resp.header(header_name.clone(), header_value.clone());
//     }

//     Ok(client_resp.body(res.body().await?))
// }

/// 代理消息业务转发
pub async fn proxy_reply(app_id: &str, req: HttpRequest, _body: web::Bytes) -> Result<HttpResponse> {
    // use crate::cluster::get_domain;
    // use wechat_sdk::Client;
    let mut domain = "ds".to_owned(); // get_domain(app_id.to_owned());

    if domain.is_empty() {
        domain = "http://366kmpf.com".to_owned();
    }

    // 创建一个可变的url地址
    let mut new_url = Url::parse(&format!("{}/WxCallback.axd", domain)).unwrap();
    new_url.set_query(req.uri().query());

    // match Client::new().post_betyes(new_url.as_str(), body).await {
    //     Ok(res) => {
    //         return Ok(HttpResponse::build(StatusCode::OK)
    //             .content_type("text/html; charset=utf-8")
    //             .body(res))
    //     }
    //     Err(err) => println!("{:?}", err),
    // };

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("success"))
}

/// 全网发布
#[allow(dead_code)]
pub async fn global_publish(
    dic: HashMap<String, String>,
    post_str: String,
) -> Result<HttpResponse> {
    log!("--- callback --- {:?}, {:?}", dic, post_str);

    let nonce = utils::get_hash_value(&dic, "nonce");
    // 对获取的消息内容进行解密
    let conf: TripartiteConfig = get_tripartite_config();
    let c = WeChatCrypto::new(&conf.token, &conf.encoding_aes_key, &conf.app_id);

    // 对接收的消息进行解码判断
    if let Ok(decode_msg) = c.decrypt_message(&post_str, &dic) {
        // println!("=== decode message === {}", decode_msg);
        let msg = Message::parse(&decode_msg);
        let to_user = msg.get_to_user();

        // 全网发布时的测试用户
        if to_user == "gh_3c884a361561" || to_user == "gh_8dad206e9538" {
            let tripart_config: TripartiteConfig = get_tripartite_config();
            let redis_config:RedisConfig=get_redis_conf();
            let comp=Component::new(tripart_config.clone(),redis_config.clone());

            match msg {
                Message::TextMessage(ref m) => {
                    // 公网发布的授权消息处理
                    if m.content.starts_with("QUERY_AUTH_CODE:") {
                        let auth_code = m.content.replace("QUERY_AUTH_CODE:", "");
                        // 根据授权码获取公众号对应的accesstoken
                        match comp.query_auth(&auth_code).await {
                            Ok(v) => {
                                // v 是一个Json对象,从json对象中获取授权 authorizer_access_token
                                if v["authorization_info"].is_object() {
                                    let auth_access_token = match v["authorization_info"]
                                        ["authorizer_access_token"]
                                        .as_str()
                                    {
                                        Some(token) => token.to_string(),
                                        None => "".to_owned(),
                                    };
                                    let kf = KFService::new(&auth_access_token);

                                    kf.send(
                                        &m.from_user,
                                        &"text".to_string(),
                                        &format!("{}_from_api", auth_code),
                                    )
                                    .await;
                                }
                            }
                            Err(e) => log!("query auth_code error: {:?}", e),
                        };
                    }
                    // 文本消息回复处理
                    else if m.content == "TESTCOMPONENT_MSG_TYPE_TEXT" {
                        let tr = TextReply::new(
                            &m.to_user,
                            &m.from_user,
                            &format!("{}_callback", &m.content),
                        );
                        let txt = tr.render();
                        log!(
                            "---- send TESTCOMPONENT_MSG_TYPE_TEXT xml :{}",
                            txt
                        );
                        let timestamp = current_timestamp();
                        let encrypt_text = c.encrypt_message(&txt, timestamp, &nonce);

                        return Ok(HttpResponse::build(StatusCode::OK)
                            .content_type("text/html; charset=utf-8")
                            .body(encrypt_text.unwrap()));
                    }
                    //其他消息
                    else {
                        let tr = TextReply::new(
                            &m.to_user,
                            &m.from_user,
                            &format!("{}_callback", &m.content),
                        );
                        let txt = tr.render();
                        let timestamp = current_timestamp();
                        let encrypt_text = c.encrypt_message(&txt, timestamp, &nonce);
                        log!("---- send OTHER xml :{}", txt);
                        return Ok(HttpResponse::build(StatusCode::OK)
                            .content_type("text/xml; charset=utf-8")
                            .body(encrypt_text.unwrap()));
                    }
                }
                Message::EventMessage(ref m) => {
                    log!("**** EVENT *** {:?}", m);
                }
                Message::UnknownMessage(ref m) => {
                    log!("**** Unknown *** {:?}", m);
                }
            }
        } else {
            use super::wx_msg;
            return wx_msg::message_reply(&msg).await;
        }
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("success"))
}
